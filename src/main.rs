#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Local};
use rsntp::SntpClient;
use slint::{SharedString, Timer, TimerMode};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_check_ntp_server({
        move || {
            println!("check button clicked...");
            let ui = ui_handle.unwrap();
            ui.set_check_button_enabled(false);
            ui.set_check_result(SharedString::from(""));
            let ntp_server = ui.get_ntp_server();
            println!("{}", format!("ntp_server1: {}", ntp_server));

            let ui_handle = ui.as_weak();
            thread::spawn(move || {
                println!("{}", format!("ntp_server2: {}", ntp_server));
                let mut client = SntpClient::new();
                client.set_timeout(Duration::from_secs(3));
                let result = client.synchronize(ntp_server.to_string());
                let mut result_str = "失败".to_string();
                match result {
                    Ok(result) => {
                        let local_time: DateTime<Local> =
                            DateTime::from(result.datetime().into_chrono_datetime().unwrap());
                        result_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    }
                    Err(_) => {}
                }

                let _ = slint::invoke_from_event_loop(move || {
                    let ui = ui_handle.clone().unwrap();
                    ui.set_check_result(SharedString::from(format!("测试结果: {}", result_str)));
                    ui.set_check_button_enabled(true);
                });
            });
        }
    });

    let ui_handle = ui.as_weak();
    /* thread::spawn(move || {
         loop {
             let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
             if let Some(ui) = ui_handle.upgrade() {
                 ui.set_time(SharedString::from(time.clone()));
                 // 等待1秒
                 thread::sleep(Duration::from_secs(1));
                 println!("22222222222222222");
             } else {
                 // 如果 UI 已经关闭，退出循环
                 println!("3333333333333333333333333333");
                 break;
             }
         }
     });*/
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        // println!("This will be printed every 500ms.");
        let ui = ui_handle.unwrap();
        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        ui.set_time(SharedString::from(time.clone()));
    });

    ui.run()
}
