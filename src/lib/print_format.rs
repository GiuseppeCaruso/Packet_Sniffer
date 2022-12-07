use crate::lib::report_packet::Report;
use pktparse::ip::IPProtocol;
use std::fs::File;
use std::io::Write;

#[warn(dead_code)]
/*pub fn print(packet: ReportPacket) {
    println!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\
                -> {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} \
                | {:?}:{:?} -> {:?}:{:?} | l3 protocol: {:?} | l4 protocol: {:?}",
        packet.source_mac.0[0],
        packet.source_mac.0[1],
        packet.source_mac.0[2],
        packet.source_mac.0[3],
        packet.source_mac.0[4],
        packet.source_mac.0[5],
        packet.dest_mac.0[0],
        packet.dest_mac.0[1],
        packet.dest_mac.0[2],
        packet.dest_mac.0[3],
        packet.dest_mac.0[4],
        packet.dest_mac.0[5],
        packet.source_ip,
        packet.source_port,
        packet.dest_ip,
        packet.dest_port,
        packet.l3_protocol,
        packet.l4_protocol
    );
}*/

pub fn fmt_for_file(packet: &Report, file: &mut File) {
    let string;
    if packet.l4_protocol == IPProtocol::Other(0) {
        string = format!(
            "{:?} -> {:?} | {:?} | bytes : {} | first_exchange : {:.3} | last_exchange : {:.3}\n\n",
            packet.source_ip,
            packet.dest_ip,
            packet.l3_protocol,
            packet.bytes_exchanged,
            packet.timestamp_first,
            packet.timestamp_last
        );
    } else if packet.l7_protocol != "".to_string() {
        string = format!(
            "{:?} -> {:?} | {:?} ({:?} -> {:?}) | {} | bytes : {} | first_exchange : {:.3} | last_exchange : {:.3}\n\n",
            packet.source_ip, packet.dest_ip, packet.l4_protocol, packet.source_port, packet.dest_port, packet.l7_protocol, packet.bytes_exchanged, packet.timestamp_first, packet.timestamp_last
        )
    } else {
        string = format!(
            "{:?} -> {:?} | {:?} ({:?} -> {:?}) | bytes : {} | first_exchange : {:.3} | last_exchange : {:.3}\n\n",
            packet.source_ip, packet.dest_ip, packet.l4_protocol, packet.source_port, packet.dest_port, packet.bytes_exchanged, packet.timestamp_first, packet.timestamp_last
        )
    }

    let res = file.write_all(string.as_bytes());
    match res {
        Ok(_) => {}
        Err(_) => {
            println!("Error! Not possible to write on file...");
        }
    }
}
