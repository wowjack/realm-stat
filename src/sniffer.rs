#![allow(dead_code)]

use std::path::PathBuf;
use etherparse::SlicedPacket;
use pcap::Device;
use crate::packet_factory::RotmgPacketFactory;


pub struct Sniffer {
    device: Device,
    factory: RotmgPacketFactory,
}
impl Sniffer {
    pub fn ask_for_device() -> Self {
        let devices = pcap::Device::list().expect("device list failed");
        for (ind, d) in devices.iter().enumerate() {
            println!("{ind}: {}", d.clone().desc.unwrap_or("Error".to_string()));
        }
        let mut selected = std::usize::MAX;
        while selected >= devices.len() {
            let mut input = String::new();
            print!("Select Network Adapter: ");
            let _ = std::io::Write::flush(&mut std::io::stdout());
            let _ = std::io::stdin().read_line(&mut input);
            if let Ok(ind) = input.trim_end().parse::<usize>() {
                selected = ind;
            }
        }
        
        let device = devices[selected as usize].clone();

        Self {
            device,
            factory: RotmgPacketFactory::new()
        }
    }



    /**
     * Open the capture handle, set the filter, and begin listening for packets and sending them to the packet factory
     */
    pub fn start(&mut self) {
        let mut received_nonmax_packet = false; //Whether or not a packet smaller than the maximum size has been received

        let mut cap = pcap::Capture::from_device(self.device.clone())
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        cap.filter(&format!("ip dst {} and ip proto \\tcp and src port 2050", self.device.addresses[0].addr.to_string()), false).expect("Error with packet filter");

        loop {
            if let Ok(p) = cap.next_packet() {
                let slice = etherparse::SlicedPacket::from_ethernet(p.data);
                match slice {
                    Ok(s) => self.process_slice(s, &mut received_nonmax_packet),
                    Err(e) => println!("Packet data error: {}", e)
                }
            } else {
                println!("unknown pcap error");
                return;
            }
            loop {
                match self.factory.get_packet() {
                    None => break,
                    Some(p) => {
                        match p {
                            //RotmgPacket::NewTick { .. } => println!("{}", self.factory.constructor.cipher.offset),
                            //RotmgPacket::Text { name, content, .. } => println!("{name}: {content}"),
                            //RotmgPacket::Damage { .. } => log::debug!("{:?}", p),
                            other => log::debug!("{:?}", other),
                            //_ => (),
                        }
                    }
                }
            }
        }
    }

    pub fn start_using_file(&mut self, path: PathBuf) {
        if path.is_file() == false {
            log::debug!("Input file does not exist");
            return;
        }

        let mut received_nonmax_packet = false;

        let mut cap = match pcap::Capture::from_file(path) {
            Ok(c) => c,
            Err(e) => {
                println!("pcap capture error: {}", e);
                return;
            }
        };
        cap.filter("ip proto \\tcp and src port 2050", false).unwrap();

        loop {
            //read and process packets
            match cap.next_packet() {
                Err(e) => {
                    log::debug!("Error fetching packet: {}", e);
                    log::debug!("{} in, {} out", self.factory.packets_in, self.factory.packets_out);
                    return;
                },
                Ok(p) => {
                    let slice = etherparse::SlicedPacket::from_ethernet(p.data);
                    match slice {
                        Ok(s) => self.process_slice(s, &mut received_nonmax_packet),
                        Err(e) => println!("Packet data error: {}", e)
                    }
                }
            }

            //check for completed packets
            loop {
                match self.factory.get_packet() {
                    None => break,
                    Some(p) => {
                        match p {
                            //RotmgPacket::NewTick { tick_id, .. } => println!("{tick_id}"),
                            //RotmgPacket::Text { name, content, .. } => println!("{name}: {content}"),
                            //RotmgPacket::Damage { .. } => log::debug!("{:?}", p),
                            other => log::debug!("{:?}", other),
                            //_ => (),
                        }
                    }
                }
            }
        }
    }

    fn process_slice(&mut self, slice: SlicedPacket, received_nonmax: &mut bool) {
        if slice.payload.len() == 0 {
            return;
        }
        if slice.payload.len() < 1460 && *received_nonmax == false {
            *received_nonmax = true;
            return;
        }
        if *received_nonmax == true {
            self.factory.insert_packet(slice);
        }
    }
}