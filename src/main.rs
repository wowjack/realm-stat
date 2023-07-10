use std::io::Write;
use byteorder::{ByteOrder};

mod rc4;
use rc4::Rc4;

use crate::sniffer::Sniffer;

mod packet_factory;

mod sniffer;

mod rotmg_packet_type;
mod byte_buffer;

fn main() {
    env_logger::init();
    //let mut bytes = [2, 48, 90, 136, 223, 169, 251, 23, 25, 102];
    //let mut rc4 = Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]);
    //rc4.apply_keystream(0, &mut bytes);

    let mut sniffer = Sniffer::ask_for_device();

    sniffer.start();

    /*
    let mut f = std::fs::File::create("./out.txt").unwrap();
    for chunk in chunks.iter() {
        let _ = f.write_all(format!("{:?}\n", chunk).as_bytes());
    }
    */
}


fn process_packet(packet: etherparse::SlicedPacket, buffer: &mut Vec<u8>) {
    let (_src_addr, _dst_addr) = match packet.ip {
        Some(etherparse::InternetSlice::Ipv4(header, _ext)) => (header.source_addr().to_string(), header.destination_addr().to_string()),
        _ => ("err".to_string(), "err".to_string())
    };

    let (src_port, _dst_port): (u16, u16) = match packet.transport {
        Some(etherparse::TransportSlice::Tcp(header)) => (header.source_port(), header.destination_port()),
        _ => (0, 0)
    };

    //println!("{}:{} tp {}:{}", src_addr, src_port, dst_addr, dst_port);
    //println!("{} {:?}", packet.payload.len(), packet.payload);
    if src_port == 2050 {
        buffer.extend_from_slice(packet.payload)
    }
}




/*
    Separates a chunk slice into slices based on the leading size number
 */
fn chunkize(mut bytes: &[u8]) -> Vec<&[u8]> {
    let mut chunks = vec![];
    loop {
        let count = get_leading_u32(bytes);
        if count as usize > bytes.len() {return chunks}
        let (c1, c2) = bytes.split_at(count as usize);
        chunks.push(c1);
        bytes = c2;
    }
}

fn get_leading_u32(bytes: &[u8]) -> u32 {
    /*
    if bytes.len() < 4 {return Err(())}
    let b: Result<[u8; 4], _> = bytes[0..4].try_into();
    return match b {
        Err(_) => return Err(()),
        Ok(s) => Ok(u32::from_be_bytes(s))
    }
    */
    byteorder::BigEndian::read_u32(bytes)
}