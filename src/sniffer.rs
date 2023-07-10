use std::io::Write;

use etherparse::SlicedPacket;
use pcap::{Capture, Active, Inactive, Device};

use crate::{packet_factory, byte_buffer};



pub struct Sniffer {
    device: Device,
    factory: packet_factory::RotmgPacketFactory,
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
            factory: packet_factory::RotmgPacketFactory::new()
        }
    }

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
            }
            loop {
                match self.factory.get_packet() {
                    None => break,
                    Some(p) => {
                        if p.packet_type != 92 {continue}
                        let bytes: Vec<u8> =  p.payload.into_iter().skip(5).collect();
                        let mut buf = byte_buffer::ByteBuffer::new(bytes);
                        let width = buf.read_u32();
                        let height = buf.read_u32();
                        let name = buf.read_string();
                        let _ = buf.read_string();
                        let realm_name = buf.read_string();
                        let seed = buf.read_u32();
                        let background = buf.read_u32();
                        let difficulty = buf.read_f32();
                        let allow_teleport = buf.read_u8();
                        let show_display = buf.read_u8();
                        let _ = buf.read_u8();
                        let max_players = buf.read_u16();
                        let _opened_time = buf.read_u32();
                        let build_version = buf.read_string();
                        let _ = buf.read_u32();
                        let dungeon_mods = buf.read_string();
                        println!("width:{:?} height:{:?} name:{:?} realm:{:?} seed:{:?} background:{:?} difficulty:{:?} tele:{:?} display:{:?} max:{:?} build:{:?} mods:{:?}",
                            width, height, name, realm_name, seed, background, difficulty, allow_teleport, show_display, max_players, build_version, dungeon_mods); 
                    }
                }
            }
        }
    }

    fn process_slice(&mut self, slice: SlicedPacket, received_nonmax: &mut bool) {
        if slice.payload.len() == 0 {
            return;
        }
        if slice.payload.len() < 1460 {
            *received_nonmax = true;
            //return;
        }
        if *received_nonmax == true {
            self.factory.insert_packet(slice);
        }
    }
}