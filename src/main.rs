use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on 127.0.0.1:7878");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs.log")
            .unwrap();

        let mut writer = BufWriter::new(file);

        for msg in rx {
            writeln!(writer, "{}", msg).unwrap();
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

fn handle_connection(stream: TcpStream, sender: mpsc::Sender<String>) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        if let Ok(text) = line {
            if sender.send(text).is_err() {
                break;
            }
        }
    }
}