use crate::lib::parsing::parse;
use crate::lib::report_packet::{Report, ReportPacket};
use pcap::Device;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn sniff(
    net_adapter: usize,
    report_vector: Arc<Mutex<Vec<Report>>>,
    filter: String,
    /*rx_sniffer: &Receiver<String>,*/ rev_tx_sniffer: Sender<String>,
    time : Instant,
    start_time : u128,
) -> Sender<String> {
    /****************** SNIFFING THREAD *******************/
    let (tx_sniffer, rx_sniffer) = channel::<String>();
    thread::Builder::new()
        .name("sniffer".into())
        .spawn(move || {
            let list = Device::list().unwrap();
            let mut cap = pcap::Capture::from_device(list[net_adapter - 1].clone())
                .unwrap()
                .promisc(true)
                .open()
                .unwrap();

            if !filter.as_str().is_empty(){
                match cap.filter(filter.as_str(), false){
                    Ok(_) => {println!("OK");},
                    Err(e) => { println!("ERROREEEEEEEEEEEEEEEEEEEEE"); },
                }
            }
            rev_tx_sniffer.send(String::from("sniffer ready!")).unwrap();
            while let Ok(packet) = cap.next_packet() {
                let handle = rx_sniffer.try_recv();
                //println!("reader: {:?}", handle);
                match handle {
                    Ok(_) => {
                        break;
                    }
                    Err(error) => {
                        if error != TryRecvError::Empty && error != TryRecvError::Disconnected {
                            println!("Unexpected error in sniffer thread...{}", error);
                        }
                    }
                };
                let report = parse(packet, time, start_time).clone();
                let report_vector_copy = report_vector.clone();
                thread::Builder::new()
                    .name("reporter".into())
                    .spawn(move || {
                        insert_into_report(&report_vector_copy, report);
                    })
                    .unwrap();
            }
            rev_tx_sniffer
                .send(String::from("Stopping sniffer thread"))
                .unwrap();
        })
        .unwrap();
    /******************************************************/
    tx_sniffer
}

pub fn insert_into_report(report_vector: &Arc<Mutex<Vec<Report>>>, packet: ReportPacket) -> () {
    let mut vec = report_vector.lock().unwrap();
    let mut found = false;
    vec.iter_mut().for_each(|p| {
        if p.source_ip == packet.source_ip
            && p.source_port == packet.source_port
            && p.dest_ip == packet.dest_ip
            && p.dest_port == packet.dest_port
        {
            p.bytes_exchanged = p.bytes_exchanged + packet.bytes_exchanged;
            if p.timestamp_first == 0.0 {
                p.timestamp_first = packet.timestamp;
                p.timestamp_last = packet.timestamp;
            } else {
                p.timestamp_last = packet.timestamp;
            }
            p.l3_protocol = packet.l3_protocol;
            p.l4_protocol = packet.l4_protocol;
            found = true;
        }
    });
    if !found {
        let l3_protocol = packet.l3_protocol;
        let l4_protocol = packet.l4_protocol;
        let report_to_insert = Report::new(
            l3_protocol,
            packet.source_ip,
            packet.dest_ip,
            l4_protocol,
            packet.source_port,
            packet.dest_port,
            packet.bytes_exchanged,
            packet.timestamp,
            packet.timestamp,
        );
        vec.push(report_to_insert)
    }
}
