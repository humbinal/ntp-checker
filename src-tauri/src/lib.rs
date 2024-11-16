use chrono::{DateTime, Local};
use rsntp::SntpClient;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) mod ntpserver;

use crate::ntpserver::{NtpServer, NtpServerController};

#[derive(Debug)]
struct NtpPacket {
    // NTP timestamp is represented as 64-bit, 32-bit seconds and 32-bit fraction.
    seconds: u32,
    fraction: u32,
}

impl NtpPacket {
    fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seconds = now.as_secs() as u32;
        let fraction = ((now.subsec_nanos() as u64 * 0xFFFFFFFF) / 1_000_000_000) as u32;
        NtpPacket { seconds, fraction }
    }

    fn to_bytes(&self) -> [u8; 48] {
        let mut buffer = [0u8; 48];

        // NTP Header is 48 bytes long
        buffer[0] = 0x1C; // LI=0, VN=4, Mode=4 (Server)

        // Time information (64-bit timestamp, 32-bit seconds and 32-bit fraction)
        buffer[43..47].copy_from_slice(&self.seconds.to_be_bytes());
        // buffer[47..51].copy_from_slice(&self.fraction.to_be_bytes());

        buffer
    }
}

#[tauri::command]
fn ntp_check(address: &str) -> Result<String, String> {
    let mut client = SntpClient::new();
    client.set_timeout(Duration::from_secs(3));
    let result = client.synchronize(address.to_string());
    match result {
        Ok(result) => {
            let local_time: DateTime<Local> =
                DateTime::from(result.datetime().into_chrono_datetime().unwrap());
            let res_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(format!(
                "connect to server {} succeed!\ngot time: {}",
                address, res_time
            ))
        }
        Err(error) => Err(format!("connect to server {} failed!\n{}", address, error)),
    }
}

#[tauri::command]
fn start_ntp_server2(state: tauri::State<Arc<Mutex<NtpServerController>>>) -> Result<(), String> {
    let addr = "0.0.0.0:123"; // NTP default port

    let socket = match UdpSocket::bind(addr) {
        Ok(socket) => socket,
        Err(e) => {
            println!("无法绑定到地址 {}: {}", addr, e);
            return Err(format!("无法绑定到地址 {}: {}", addr, e));
        }
    };

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        println!("NTP 服务器线程已启动");
        println!("NTP Server running on {}", addr);

        let mut buf = [0u8; 48]; // NTP packet size is 48 bytes
        loop {
            if rx.try_recv().is_ok() {
                println!("NTP 服务器线程接收到停止信号");
                break;
            }

            let recv_result = socket.recv_from(&mut buf);

            match recv_result {
                Ok((size, src)) => {
                    // let (size, src) = socket.recv_from(&mut buf)?;
                    println!("Received {} bytes from {}", size, src);

                    // We create an NTP packet with the current timestamp
                    let ntp_packet = NtpPacket::new();

                    // Send the NTP response back to the client
                    let response = ntp_packet.to_bytes();
                    match socket.send_to(&response, src) {
                        Ok(_) => println!("Sent response to {}", src),
                        Err(e) => eprintln!("发送响应失败: {}", e),
                    }

                    println!("Sent response to {}", src);
                }
                Err(e) => {
                    println!("Receive error: {}", e);
                }
            }
        }
        // let socket = UdpSocket::bind(addr)?;
    });

    let mut controller = state.lock().unwrap();
    controller.handle = Some(handle);
    controller.stop_sender = Some(tx);
    Ok(())
}

#[tauri::command]
fn start_ntp_server(state: tauri::State<Arc<Mutex<NtpServerController>>>) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        println!("NTP 服务器线程已启动");
        println!("NTP Server running on {}", "0.0.0.0:123");
        let server = NtpServer::new("0.0.0.0:123".to_string(), rx, true);
        server.run();
    });

    let mut controller = state.lock().unwrap();
    controller.handle = Some(handle);
    controller.stop_sender = Some(tx);
    Ok(())
}

#[tauri::command]
fn stop_ntp_server(state: tauri::State<Arc<Mutex<NtpServerController>>>) {
    let mut controller = state.lock().unwrap();
    if let Some(sender) = controller.stop_sender.take() {
        let _ = sender.send(()); // 发送停止信号
        println!("发送停止信号");
    }
    if let Some(handle) = controller.handle.take() {
        let _ = handle.join(); // 等待线程结束
        println!("NTP 服务器线程已停止");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let controller = Arc::new(Mutex::new(NtpServerController::new()));
    tauri::Builder::default()
        .manage(controller)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            ntp_check,
            start_ntp_server,
            stop_ntp_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
