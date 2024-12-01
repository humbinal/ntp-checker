use chrono::{DateTime, Local};
use rsntp::SntpClient;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri_plugin_shell::open::Program::Open;

pub(crate) mod ntpserver;

use crate::ntpserver::{NtpServer, NtpServerController};

#[derive(Serialize, Deserialize)]
struct NtpCheckSuccess {
    ip: String,
    port: i32,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct NtpCheckError {
    ip: String,
    port: i32,
    msg: String,
}

#[tauri::command]
fn ntp_check(ip: &str, port: i32) -> Result<NtpCheckSuccess, NtpCheckError> {
    let mut client = SntpClient::new();
    client.set_timeout(Duration::from_secs(3));
    let address = format!("{}:{}", ip, port);
    let result = client.synchronize(address);
    match result {
        Ok(result) => {
            let local_time: DateTime<Local> =
                DateTime::from(result.datetime().into_chrono_datetime().unwrap());
            let res_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(NtpCheckSuccess {
                ip: ip.to_string(),
                port,
                date: res_time,
            })
        }
        Err(error) => Err(NtpCheckError {
            ip: ip.to_string(),
            port,
            msg: error.to_string(),
        }),
    }
}

#[tauri::command]
fn start_ntp_server(
    state: tauri::State<Arc<Mutex<NtpServerController>>>,
    port: i32,
) -> Result<(), String> {
    let mut controller = state.lock().unwrap();
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        println!("NTP 服务器线程已启动");
        let address = format!("0.0.0.0:{}", port);
        println!("NTP Server running on {}", address);
        let server = NtpServer::new(address, rx, true);
        server.run();
    });
    controller.handle = Some(handle);
    controller.stop_sender = Some(tx);
    controller.set_running();
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
    controller.set_finished();
}

#[tauri::command]
fn get_ntp_server_state(state: tauri::State<Arc<Mutex<NtpServerController>>>) -> bool {
    let controller = state.lock().unwrap();
    controller.is_running()
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
            stop_ntp_server,
            get_ntp_server_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
