use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use super::protector::*;

const MESSAGE_SIZE: usize = 32;
const SLEEP_TIME: u64 = 100;

pub fn run(localhost:String) {
    let (mut init_hash, mut key) = (super::protector::get_hash_str(), super::protector::get_session_key());
    let mut hash_in_byte = init_hash.clone().into_bytes();
    let protect = super::protector::SessionProtector{hash: init_hash};
    let mut client = TcpStream::connect(localhost).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize nonblocking");
    let (sx, rx) = mpsc::channel::<String>();
    hash_in_byte.resize(MESSAGE_SIZE, 0);
    client.write_all(&hash_in_byte).expect("Writing to socket failed");
    thread::spawn(move || loop {
        let mut buf = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {break;}
        }
        match rx.try_recv() {
            Ok(msg) => {
                let (mut buf, mut buf_key) = (msg.clone().into_bytes(), key.clone().into_bytes());
                buf.resize(MESSAGE_SIZE, 0);
                buf_key.resize(MESSAGE_SIZE, 0);
                client.write_all(&buf_key).expect("Writing to socket failed");
                client.write_all(&buf).expect("Writing to socket failed");
                key = protect.next_session_key(&key);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(SLEEP_TIME));
    });
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Reading form stdin failed");
        let msg = buf.trim().to_string();
        if msg == ":q" || sx.send(msg).is_err() { break }
    }
}