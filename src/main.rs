use std::io::Write;


fn main() {
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

    cap.filter(&format!("dst host {}", device.addresses[0].addr.to_string()), false).expect("Error with packet filter");
    
    
    for _ in 0..10 {
        let slice = etherparse::SlicedPacket::from_ethernet(cap.next_packet().unwrap().data).expect("Error reading packet data");
            match slice.ip.expect("Failed to get IP") {
            etherparse::InternetSlice::Ipv4(header, _ext) => {
                println!("{:?} to {:?}", header.source_addr(), header.destination_addr());
            },
            _ => panic!("Not an ipv4 packet")
        }
    }
    
}
