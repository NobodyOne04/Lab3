use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const MESSAGE_SIZE: usize = 32;
const SLEEP_TIME: u64 = 100;

pub fn run(localhost:String) {
    let mut client = TcpStream::connect(localhost).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize nonblocking");
    let (sx, rx) = mpsc::channel::<String>();
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
                let mut buf = msg.clone().into_bytes();
                buf.resize(MESSAGE_SIZE, 0);
                client.write_all(&buf).expect("Writing to socket failed");
                println!("message sent {:?}", msg);
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