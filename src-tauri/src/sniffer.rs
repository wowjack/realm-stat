#![allow(dead_code)]

use pcap::Device;
use crate::packet_factory::{RotmgPacketFactory, rotmg_packet::RotmgPacket};
use std::sync::{Arc, Mutex};


pub struct Sniffer {
    device: Option<Device>,
    capture_thread: Option<std::thread::JoinHandle<()>>,
    factory: Arc<Mutex<RotmgPacketFactory>>,
    collect: Arc<Mutex<bool>>,
    session_buffer: Arc<Mutex<Vec<RotmgPacket>>>,
}
impl Sniffer {
    pub fn new() -> Self {
        Self {
            device: None,
            capture_thread: None,
            factory: Arc::new(Mutex::new(RotmgPacketFactory::new())),
            collect: Arc::new(Mutex::new(false)),
            session_buffer: Arc::new(Mutex::new(vec![])),
        }
    }

    /**
     * Open the capture handle, set the filter, and begin listening for packets and sending them to the packet factory.
     * A tauri window is required to inform the ui of changes in the cipher alignment
     */
    pub fn start(&mut self, window: tauri::Window) {
        {
            *self.collect.lock().unwrap() = true;
            self.factory.lock().unwrap().reset();
            self.session_buffer.lock().unwrap().clear();
            window.emit("cipher-misaligned", ()).unwrap();
        }
        let factory = self.factory.clone();
        let device = self.device.clone();
        let run = self.collect.clone();
        let session_buffer = self.session_buffer.clone();
        let handle = std::thread::spawn(move || {
            let mut received_nonmax_packet = false; //Whether or not a packet smaller than the maximum size has been received

            let mut cap = pcap::Capture::from_device(device.unwrap().clone())
                .unwrap()
                .immediate_mode(true)
                .timeout(1000)
                .open()
                .unwrap();

            cap.filter("ip proto \\tcp and src port 2050", false).expect("Error with packet filter");

            while *run.lock().unwrap() == true {
                log::debug!("sniffer is running");
                match cap.next_packet() {
                    Ok(p) => {
                        let slice = etherparse::SlicedPacket::from_ethernet(p.data);
                        match slice {
                            Ok(s) => {
                                if s.payload.len() == 0 {
                                    continue;
                                }
                                if s.payload.len() < 1460 && received_nonmax_packet == false {
                                    received_nonmax_packet = true;
                                    continue;
                                }
                                if received_nonmax_packet == true {
                                    let mut factory = factory.lock().expect("RwLock error");
                                    factory.insert_packet(s, &window);
                                    while let Some(p) = factory.get_packet() {
                                        session_buffer.lock().unwrap().push(p);
                                    }
                                }
                            },
                            Err(e) => println!("Packet data error: {}", e)
                        }
                    },
                    Err(e) => println!("pcap error {}", e),   
                }
            }
            //log::debug!("Collection thread stopping");
        });
        self.capture_thread = Some(handle);
    }

    /**
     * Open a capture handle to a pcap file, set the filter, and begin processing packets
     */
    pub fn start_using_pcap_file(&mut self, window: tauri::Window, file_path: String) {
        {
            *self.collect.lock().unwrap() = true;
            self.factory.lock().unwrap().reset();
            self.session_buffer.lock().unwrap().clear();
        }
        let factory = self.factory.clone();
        let session_buffer = self.session_buffer.clone();
        let handle = std::thread::spawn(move || {
            let mut received_nonmax_packet = false; //Whether or not a packet smaller than the maximum size has been received

            let mut cap = match pcap::Capture::from_file(file_path) {
                Err(e) => {
                    log::debug!("{:?}", e);
                    return;
                },
                Ok(c) => c
            };

            if let Err(e) = cap.filter("ip proto \\tcp and src port 2050", false) {
                log::debug!("{:?}", e);
                return;
            }

            loop {
                match cap.next_packet() {
                    Err(_) => break,
                    Ok(p) => {
                        let slice = etherparse::SlicedPacket::from_ethernet(p.data);
                        match slice {
                            Ok(s) => {
                                if s.payload.len() == 0 {
                                    continue;
                                }
                                if s.payload.len() < 1460 && received_nonmax_packet == false {
                                    received_nonmax_packet = true;
                                    continue;
                                }
                                if received_nonmax_packet == true {
                                    let mut factory = factory.lock().expect("RwLock error");
                                    factory.insert_packet(s, &window);
                                    while let Some(p) = factory.get_packet() {
                                        session_buffer.lock().unwrap().push(p);
                                    }
                                }
                            },
                            Err(e) => println!("Packet data error: {}", e)
                        }
                    },   
                }
            }
            //log::debug!("Collection thread stopping");
            window.emit("pcap-eof", ()).expect("Error emitting event");
        });
        self.capture_thread = Some(handle);
    }

    pub fn stop(&mut self) {
        *self.collect.lock().unwrap() = false;
        if let Some(jh) = self.capture_thread.take() {
            let _ = jh.join();
        }
        //log::debug!("Collection stopped");

    }

    pub fn log_packets(&mut self) {
        loop {
            match self.factory.lock().unwrap().get_packet() {
                None => break,
                Some(p) => log::debug!("{:?}", p),
            }
        }
    }

    pub fn get_all_packets(&mut self) -> Vec<RotmgPacket> {
        self.session_buffer.lock().unwrap().to_vec()
    }

    pub fn set_device(&mut self, device: &Device) {
        self.device = Some(device.clone());
    }
}