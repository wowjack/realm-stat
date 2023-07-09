use std::io::Write;
use byteorder::{ReadBytesExt, ByteOrder};
use rc4::{Key, Rc4, consts::*, KeyInit, StreamCipher};


fn main() {
    let b = [0u8, 0, 0, 10, 0];
    let s = get_leading_u32(&b);
    println!("{s}");
    return;

    let devices = pcap::Device::list().expect("device list failed");
    for (ind, d) in devices.iter().enumerate() {
        println!("{ind}: {}", d.clone().desc.unwrap_or("Error".to_string()));
    }
    let mut selected = std::usize::MAX;
    while selected >= devices.len() {
        let mut input = String::new();
        print!("Select Network Adapter: ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);
        if let Ok(ind) = input.trim_end().parse::<usize>() {
            selected = ind;
        }
    }
    
    let device = devices[selected as usize].clone();

    println!("Listening with addr {:?}", device.clone().addresses[0].addr);
    

    // Setup Capture
    let mut cap = pcap::Capture::from_device(device.clone())
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    cap.filter(&format!("ip dst {} and ip proto \\tcp", device.addresses[0].addr.to_string()), false).expect("Error with packet filter");
    
    
    let mut bytebuffer: Vec<u8> = vec![];
    while bytebuffer.len() < 50000 {
        let new_packet = cap.next_packet();
        if let Ok(packet) = new_packet {
            let slice = etherparse::SlicedPacket::from_ethernet(packet.data);
            match slice {
                Ok(s) => process_packet(s, &mut bytebuffer),
                Err(e) => println!("Packet data error: {}", e)
            }
        }
    }

    let mut f = std::fs::File::create("./out.txt").unwrap();
    let chunks = chunkize(bytebuffer.as_slice());
    for chunk in chunks.iter() {
        let _ = f.write_all(format!("{:?}\n", chunk).as_bytes());
    }

    println!("{:?}", chunks[0]);


    /*
    let mut rc4 = Rc4::new(b"c91d9eec420160730d825604e0".into());
    let mut data = chunks[0].to_owned();
    rc4.apply_keystream(&mut data);
    println!("{}", String::from_utf8(data).unwrap());
    */
}


fn process_packet(packet: etherparse::SlicedPacket, buffer: &mut Vec<u8>) {
    let (src_addr, dst_addr) = match packet.ip.expect("Error getting ip header") {
        etherparse::InternetSlice::Ipv4(header, _ext) => (header.source_addr().to_string(), header.destination_addr().to_string()),
        _ => ("err".to_string(), "err".to_string())
    };

    let (src_port, dst_port): (u16, u16) = match packet.transport.expect("Error getting transport header") {
        etherparse::TransportSlice::Tcp(header) => (header.source_port(), header.destination_port()),
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