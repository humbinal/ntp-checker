#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Local};
use rsntp::SntpClient;
use slint::{SharedString, Timer, TimerMode};

use winit_helper::center_window;

mod winit_helper;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.show()?;

    center_window(ui.window());

    let ui_handle = ui.as_weak();

    ui.on_check_ntp_server({
        move || {
            println!("check button clicked...");
            let ui = ui_handle.unwrap();
            ui.set_check_button_enabled(false);
            let ntp_server = ui.get_ntp_server();
            ui.set_check_result(SharedString::from(format!("服务器{}连接中...", ntp_server)));
            let ui_handle = ui.as_weak();
            thread::spawn(move || {
                let mut client = SntpClient::new();
                client.set_timeout(Duration::from_secs(3));
                let result = client.synchronize(ntp_server.to_string());
                let (result_msg, succeed) = match result {
                    Ok(result) => {
                        let local_time: DateTime<Local> =
                            DateTime::from(result.datetime().into_chrono_datetime().unwrap());
                        let res_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                        (format!("服务器{}连接成功!\n返回时间为: {}", ntp_server, res_time), true)
                    }
                    Err(error) => {
                        (format!("服务器{}连接失败!\n{}", ntp_server, error), false)
                    }
                };

                let _ = slint::invoke_from_event_loop(move || {
                    let ui = ui_handle.clone().unwrap();
                    ui.set_check_succeed(succeed);
                    ui.set_check_result(SharedString::from(result_msg));
                    ui.set_check_button_enabled(true);
                });
            });
        }
    });

    let ui_handle = ui.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(200), move || {
        // println!("This will be printed every 200ms.");
        let ui = ui_handle.unwrap();
        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        ui.set_time(SharedString::from(time.clone()));
    });

    ui.run()
}
