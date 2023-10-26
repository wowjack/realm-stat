// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::sync::{Mutex, Arc};

use packet_factory::rotmg_packet::RotmgPacket;
use tauri::Window;

use crate::sniffer::Sniffer;

mod rc4;
mod packet_factory;
mod sniffer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn start_collection(sniffer: tauri::State<Arc<Mutex<Sniffer>>>, window: Window) {
    //log::debug!("Starting collection"); 
    sniffer.lock().unwrap().start();
}

#[tauri::command]
fn start_pcap(sniffer: tauri::State<Arc<Mutex<Sniffer>>>, window: Window, file_path: String) {
    sniffer.lock().unwrap().start_using_pcap_file(file_path.clone(), window);
    //log::debug!("{}", file_path);
}

#[tauri::command]
fn stop_collection(sniffer: tauri::State<Arc<Mutex<Sniffer>>>) {
    //log::debug!("Stopping collection");
    sniffer.lock().unwrap().stop(); 
}

#[tauri::command]
fn get_packets(sniffer: tauri::State<Arc<Mutex<Sniffer>>>) -> Vec<RotmgPacket> {
    //log::debug!("Fetching packets");
    let p = sniffer.lock().unwrap().get_all_packets();
    //log::debug!("{:?}", p);
    return p
}

#[tauri::command]
fn get_devices() -> Vec<String> {
    return pcap::Device::list().expect("device list failed").iter().map(|d| d.desc.clone().unwrap_or("error".to_string())).collect();
}
#[tauri::command]
fn use_device(sniffer: tauri::State<Arc<Mutex<Sniffer>>>, device_string: String) -> Result<(), ()> {
    match pcap::Device::list().expect("Device list failed").iter().find(|d| d.desc == Some(device_string.clone())) {
        None => return Err(()),
        Some(d) => sniffer.lock().unwrap().set_device(d),
    }
    return Ok(())
}

fn main() {
    //let _ = simple_logging::log_to_file("log.log", log::LevelFilter::Debug);
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(Sniffer::new())))
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            start_collection,
            start_pcap,
            stop_collection,
            get_packets,
            get_devices,
            use_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


