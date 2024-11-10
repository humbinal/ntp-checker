use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use rsntp::SntpClient;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::time::Duration;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ntp_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
