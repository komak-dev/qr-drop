// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut is_frontend_dev = false;

    for arg in args[1..].iter() {
        if arg == "--frontend-dev" {
            is_frontend_dev = true;
        }
    }

    qr_drop_lib::run()
}
