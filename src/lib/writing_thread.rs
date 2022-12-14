use crate::lib::print_format::fmt_for_file;
use crate::lib::report_packet::Report;
use std::fs::{File, OpenOptions};
use std::io::Seek;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn write_file(
    file_name: String,
    timeout: u16,
    report_vector: Arc<Mutex<Vec<Report>>>,
    rev_tx_writer: Sender<String>,
) -> Sender<String> {
    /****************** WRITER THREAD *******************/
    let (tx_writer, rx_writer) = channel::<String>();
    thread::Builder::new()
        .name("writer".into())
        .spawn(move || {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(file_name)
                .expect("Impossible to open the file! Press stop to end the program."); //file opened in append mode, read-write mode, if not exists, create it
            rev_tx_writer.send(String::from("writer ready!")).unwrap();
            loop {
                let handle = rx_writer.try_recv();

                match handle {
                    Ok(_) => {
                        break;
                    }
                    Err(error) => {
                        if error != TryRecvError::Empty && error != TryRecvError::Disconnected {
                            panic!("Unexpected error! Press stop to end the program.");
                        }
                    }
                };
                write_report(&report_vector, timeout as u64, &mut file);
            }
            rev_tx_writer
                .send(String::from("Stopping writer thread"))
                .expect("Unexpected error! Press stop to end the program.");
        })
        .expect("Unexpected error! Press stop to end the program.");
    /******************************************************/
    tx_writer
}

pub fn write_report(report_vector: &Arc<Mutex<Vec<Report>>>, timeout: u64, file: &mut File) -> () {
    file.rewind()
        .expect("Unexpected error! Press stop to end the program.");
    thread::sleep(Duration::from_secs(timeout));

    let vec = report_vector
        .lock()
        .expect("Unexpected error! Press stop to end the program.");
    vec.iter().for_each(|p| fmt_for_file(p, file));
}
