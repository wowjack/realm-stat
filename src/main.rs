fn main() {
    let devices = pcap::Device::list().expect("device list failed");
    let device = devices.iter().filter_map(|d| if d.desc == Some("802.11ac Wireless LAN Card".to_string()) {Some(d)} else {None}).next().expect("No device found").clone();
    

    // Setup Capture
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    // get a packet and print its bytes
    println!("{:?}", cap.next_packet());

}
