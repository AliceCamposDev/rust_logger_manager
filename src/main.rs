mod domain;

use crate::domain::log_event::BaseLog;
use serde_json;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on 127.0.0.1:7878");

    let (tx, rx) = mpsc::channel::<BaseLog>();

    thread::spawn(move || {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs.log")
            .unwrap();

        let mut writer = BufWriter::new(file);

        for log in rx {
            let json = serde_json::to_string(&log).unwrap();
            writeln!(writer, "{}", json).unwrap();
            writer.flush().unwrap();
        }
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            handle_connection(stream, tx_clone);
        });
    }
}

fn handle_connection(stream: TcpStream, sender: mpsc::Sender<BaseLog>) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        if let Ok(text) = line {
            match serde_json::from_str::<BaseLog>(&text) {
                Ok(log) => {
                    if sender.send(log).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Deserialize error: {:?}", e);
                    println!("RAW: {}", text);
                }
            }
        }
    }
}
