// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod ntpserver;

fn main() {
    tauri_app_lib::run()
}
