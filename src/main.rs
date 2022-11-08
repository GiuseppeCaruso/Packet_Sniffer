use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use pcap::{Device, Error, Packet};
use pktparse::ethernet::{EtherType, MacAddress};
use pktparse::ip::IPProtocol;
use crate::report_packet::report_packet::ReportPacket;

//main di test
mod cli;
mod lib;
mod report_packet;
mod sniffing_thread;
mod parsing;
mod print_format;
mod writing_thread;

fn main() {
    /*******************READING FROM CLI******************/

    let mut args = cli::get_cli();
    let net_adapter_cp = args.net_adapter.clone();
    let output_file_name = args.output_file_name.clone();
    let filter = args.filter.clone();
    let timeout = args.timeout.clone();
    let mut report_vector = Arc::new(Mutex::new(Vec::new()));

    /*  args's arguments:
        net_adapter: index used in selecting Device::lookup
        output_file_name: self explained
        filter: string to filter packet sniffing
        timeout: time after which a report must be produced */
    /******************************************************/

    /****************** SNIFFING THREADS *******************/
    let filter = filter.clone();
    let report_vector2 = report_vector.clone();
    sniffing_thread::sniff(net_adapter_cp, report_vector2, filter);

    /******************* WRITING THREAD *******************/

    let report_vector1 = report_vector.clone();
    writing_thread::write_file(output_file_name, timeout, report_vector1);

    /******************************************************/
    loop{

    }
    /******************************************************/

    /*********** READING PACKETS SENT BY THE SNIFFING THREADS *************/


    /*********************************************************************/
}