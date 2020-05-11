use std::io::prelude::*;
use std::net::TcpStream;
use std::{io, thread};
use std::time::Duration;
use std::sync::mpsc::channel;


fn main() {
    let mut stream = TcpStream::connect("irc.mindfang.org:6667").expect("Failed!");
    let mut readbuffer = String::new();
    //let mut writebuffer = String::new();
    let _ = stream.set_read_timeout(Some(Duration::from_millis(20)));
    let mut running: bool = true;

    let mut writebuffer = String::new();

    let (sender, receiver) = channel();

    let t = thread::spawn(move || {
        let mstdin = io::stdin();


        while running {
            let mut tbuffer = String::new();
            mstdin.read_line(&mut tbuffer).unwrap();
            sender.send(String::from(tbuffer.trim_end())).unwrap();
            sender.send(String::from("\r\n")).unwrap();
        }
    });

    while running {
        while let Ok(popped) = receiver.try_recv() {
            writebuffer.push_str(&popped);
        }

        if let Some(index) = writebuffer.find("\r\n") {
            writebuffer = {
                let (msg, rest) = writebuffer.split_at(index);
                if msg == "QUIT\r\n" {
                    running = false;
                    continue
                }
                stream.write(msg.as_bytes()).unwrap();
                rest.to_string()
            };
        }

        let _ = stream.read_to_string(&mut readbuffer);
        if let Some(index) = readbuffer.find("\r\n") {
            readbuffer = {
                let (msg, rest) = readbuffer.split_at(index + 2);
                println!("{}", msg.trim_end());
                rest.to_string()
            }
        }
    }

    t.join().expect("Failed to join??");
}
