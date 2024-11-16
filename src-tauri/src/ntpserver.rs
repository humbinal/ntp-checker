extern crate byteorder;
extern crate net2;
extern crate rand;

use byteorder::{BigEndian, ByteOrder};
use std::io;
use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use net2::UdpBuilder;

use rand::random;

#[derive(Debug, Copy, Clone)]
struct NtpTimestamp {
    ts: u64,
}

impl NtpTimestamp {
    fn now() -> NtpTimestamp {
        let now = SystemTime::now();
        let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs = dur.as_secs() + 2208988800; // 1900 epoch
        let nanos = dur.subsec_nanos();

        NtpTimestamp {
            ts: (secs << 32) + (nanos as f64 * 4.294967296) as u64,
        }
    }

    fn zero() -> NtpTimestamp {
        NtpTimestamp { ts: 0 }
    }

    fn random() -> NtpTimestamp {
        NtpTimestamp { ts: random() }
    }

    fn diff_to_sec(&self, ts: &NtpTimestamp) -> f64 {
        (self.ts.wrapping_sub(ts.ts)) as i64 as f64 / 4294967296.0
    }

    fn read(buf: &[u8]) -> NtpTimestamp {
        NtpTimestamp {
            ts: BigEndian::read_u64(buf),
        }
    }

    fn write(&self, buf: &mut [u8]) {
        BigEndian::write_u64(buf, self.ts);
    }
}

impl PartialEq for NtpTimestamp {
    fn eq(&self, other: &NtpTimestamp) -> bool {
        self.ts == other.ts
    }
}

#[derive(Debug, Copy, Clone)]
struct NtpFracValue {
    val: u32,
}

impl NtpFracValue {
    fn read(buf: &[u8]) -> NtpFracValue {
        NtpFracValue {
            val: BigEndian::read_u32(buf),
        }
    }

    fn write(&self, buf: &mut [u8]) {
        BigEndian::write_u32(buf, self.val);
    }

    fn zero() -> NtpFracValue {
        NtpFracValue { val: 0 }
    }

    fn increment(&mut self) {
        self.val += 1;
    }
}

#[derive(Debug)]
struct NtpPacket {
    remote_addr: SocketAddr,
    local_ts: NtpTimestamp,

    leap: u8,
    version: u8,
    mode: u8,
    stratum: u8,
    poll: i8,
    precision: i8,
    delay: NtpFracValue,
    dispersion: NtpFracValue,
    ref_id: u32,
    ref_ts: NtpTimestamp,
    orig_ts: NtpTimestamp,
    rx_ts: NtpTimestamp,
    tx_ts: NtpTimestamp,
}

impl NtpPacket {
    fn receive(socket: &UdpSocket) -> io::Result<NtpPacket> {
        let mut buf = [0; 1024];

        // let (len, addr) = socket.recv_from(&mut buf);
        match socket.recv_from(&mut buf) {
            Ok((len, addr)) => {
                let local_ts = NtpTimestamp::now();

                if len < 48 {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "Packet too short"));
                }

                let leap = buf[0] >> 6;
                let version = (buf[0] >> 3) & 0x7;
                let mode = buf[0] & 0x7;

                if version < 1 || version > 4 {
                    return Err(Error::new(ErrorKind::Other, "Unsupported version"));
                }

                Ok(NtpPacket {
                    remote_addr: addr,
                    local_ts: local_ts,
                    leap: leap,
                    version: version,
                    mode: mode,
                    stratum: buf[1],
                    poll: buf[2] as i8,
                    precision: buf[3] as i8,
                    delay: NtpFracValue::read(&buf[4..8]),
                    dispersion: NtpFracValue::read(&buf[8..12]),
                    ref_id: BigEndian::read_u32(&buf[12..16]),
                    ref_ts: NtpTimestamp::read(&buf[16..24]),
                    orig_ts: NtpTimestamp::read(&buf[24..32]),
                    rx_ts: NtpTimestamp::read(&buf[32..40]),
                    tx_ts: NtpTimestamp::read(&buf[40..48]),
                })
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                Err(Error::new(ErrorKind::WouldBlock, "Would Block"))
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn send(&self, socket: &UdpSocket) -> io::Result<usize> {
        let mut buf = [0; 48];

        buf[0] = self.leap << 6 | self.version << 3 | self.mode;
        buf[1] = self.stratum;
        buf[2] = self.poll as u8;
        buf[3] = self.precision as u8;
        self.delay.write(&mut buf[4..8]);
        self.dispersion.write(&mut buf[8..12]);
        BigEndian::write_u32(&mut buf[12..16], self.ref_id);
        self.ref_ts.write(&mut buf[16..24]);
        self.orig_ts.write(&mut buf[24..32]);
        self.rx_ts.write(&mut buf[32..40]);
        self.tx_ts.write(&mut buf[40..48]);

        socket.send_to(&buf, self.remote_addr)
    }

    fn is_request(&self) -> bool {
        self.mode == 1
            || self.mode == 3
            || (self.mode == 0 && self.version == 1 && self.remote_addr.port() != 123)
    }

    fn make_response(&self, state: &NtpServerState) -> Option<NtpPacket> {
        if !self.is_request() {
            return None;
        }

        Some(NtpPacket {
            remote_addr: self.remote_addr,
            local_ts: NtpTimestamp::zero(),
            leap: state.leap,
            version: self.version,
            mode: if self.mode == 1 { 2 } else { 4 },
            stratum: state.stratum,
            poll: self.poll,
            precision: state.precision,
            delay: state.delay,
            dispersion: state.dispersion,
            ref_id: state.ref_id,
            ref_ts: state.ref_ts,
            orig_ts: self.tx_ts,
            rx_ts: self.local_ts,
            tx_ts: NtpTimestamp::now(),
        })
    }

    fn new_request(remote_addr: SocketAddr) -> NtpPacket {
        NtpPacket {
            remote_addr,
            local_ts: NtpTimestamp::now(),
            leap: 0,
            version: 4,
            mode: 3,
            stratum: 1,
            poll: 0,
            precision: 0,
            delay: NtpFracValue::zero(),
            dispersion: NtpFracValue::zero(),
            ref_id: 0,
            ref_ts: NtpTimestamp::zero(),
            orig_ts: NtpTimestamp::zero(),
            rx_ts: NtpTimestamp::zero(),
            tx_ts: NtpTimestamp::random(),
        }
    }

    fn is_valid_response(&self, request: &NtpPacket) -> bool {
        self.remote_addr == request.remote_addr
            && self.mode == request.mode + 1
            && self.orig_ts == request.tx_ts
    }

    fn get_server_state(&self) -> NtpServerState {
        NtpServerState {
            leap: self.leap,
            stratum: self.stratum,
            precision: self.precision,
            ref_id: self.ref_id,
            ref_ts: self.ref_ts,
            dispersion: self.dispersion,
            delay: self.delay,
        }
    }
}

#[derive(Copy, Clone)]
struct NtpServerState {
    leap: u8,
    stratum: u8,
    precision: i8,
    ref_id: u32,
    ref_ts: NtpTimestamp,
    dispersion: NtpFracValue,
    delay: NtpFracValue,
}

pub struct NtpServer {
    state: Arc<Mutex<NtpServerState>>,
    sockets: Vec<UdpSocket>,
    rx: Receiver<()>,
    debug: bool,
}

impl NtpServer {
    fn process_requests(
        thread_id: u32,
        debug: bool,
        socket: UdpSocket,
        state: Arc<Mutex<NtpServerState>>,
        rx: Receiver<()>,
    ) {
        let mut last_update = NtpTimestamp::now();
        let mut cached_state: NtpServerState;
        cached_state = *state.lock().unwrap();

        println!("Server thread #{} started", thread_id);

        loop {
            if rx.try_recv().is_ok() {
                println!("process_requests线程接收到停止信号");
                break;
            }
            match NtpPacket::receive(&socket) {
                Ok(request) => {
                    if debug {
                        println!("Thread #{} received {:?}", thread_id, request);
                    }

                    if request.local_ts.diff_to_sec(&last_update).abs() > 0.1 {
                        cached_state = *state.lock().unwrap();
                        last_update = request.local_ts;
                        if debug {
                            println!("Thread #{} updated its state", thread_id);
                        }
                    }

                    match request.make_response(&cached_state) {
                        Some(response) => match response.send(&socket) {
                            Ok(_) => {
                                if debug {
                                    println!("Thread #{} sent {:?}", thread_id, response);
                                }
                            }
                            Err(e) => println!(
                                "Thread #{} failed to send packet to {}: {}",
                                thread_id, response.remote_addr, e
                            ),
                        },
                        None => {}
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    println!("Thread #{} failed to receive packet: {}", thread_id, e);
                }
            }
        }
    }

    pub fn new(addr: String, rx: Receiver<()>, debug: bool) -> NtpServer {
        let state = NtpServerState {
            leap: 0,
            stratum: 1,
            precision: 0,
            ref_id: 0,
            ref_ts: NtpTimestamp::zero(),
            dispersion: NtpFracValue::zero(),
            delay: NtpFracValue::zero(),
        };
        let mut sockets = vec![];
        let socket = match UdpSocket::bind(addr) {
            Ok(socket) => {
                socket.set_nonblocking(true).unwrap();
                socket
            }
            Err(e) => {
                panic!("Couldn't bind socket: {}", e)
            }
        };
        sockets.push(socket);

        NtpServer {
            state: Arc::new(Mutex::new(state)),
            sockets,
            rx,
            debug,
        }
    }

    pub fn run(&self) {
        let mut threads = vec![];
        let mut id = 0;
        let quit = false;

        let mut txs: Vec<Sender<()>> = vec![];
        for socket in &self.sockets {
            id = id + 1;
            let state = self.state.clone();
            let debug = self.debug;
            let cloned_socket = socket.try_clone().unwrap();
            let (tx, rx) = mpsc::channel();
            threads.push(thread::spawn(move || {
                NtpServer::process_requests(id, debug, cloned_socket, state, rx);
            }));
            txs.push(tx);
        }

        while !quit {
            if self.rx.try_recv().is_ok() {
                println!("rx received stop signal.");
                for tx in txs {
                    tx.send(()).unwrap();
                    println!("sent stop signal to tx: #{:?}", tx);
                }
                break;
            }
            thread::sleep(Duration::new(1, 0));
        }

        for thread in threads {
            let _ = thread.join();
        }
    }
}

pub struct NtpServerController {
    pub handle: Option<JoinHandle<()>>,
    pub stop_sender: Option<Sender<()>>,
}

impl NtpServerController {
    pub fn new() -> Self {
        Self {
            handle: None,
            stop_sender: None,
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let server = NtpServer::new("0.0.0.0:123".to_string(), rx, true);

    server.run();
}
