// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(unused)]

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

mod images;
mod record;
use images::*;
use record::*;

fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));
    tauri::Builder::default()
        .manage(stop_signal)
        .invoke_handler(tauri::generate_handler![
            start_threads,
            stop_thread,
            get_images,
            get_folder_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
