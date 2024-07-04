extern crate rlibs;

use std::net::UdpSocket;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{rc, thread};

use rlibs::fileio::file_writer::Writer;
use rlibs::socketio::udp_socket::*;

fn main() {

    udp();

}

fn udp() {

    start_send();
    match start_recv().join() {
        Ok(_) => println!("Receive Finished"),
        Err(_) => panic!("Threat Panicked"),
    }
}

fn start_recv() -> JoinHandle<()>{
    thread::spawn(|| {

        let rcv = | bytes : &[u8], sock : &UdpComms | {
            callback(bytes, sock);
        };

        let receiver = UdpComms::new(String::from("127.0.0.1"), 1833, rcv);


        receiver.start();
    })
}

fn start_send() {
    let mut sender = UdpComms::new(
        String::from("127.0.0.1"), 1832,
        |b:&[u8], s:&UdpComms|{callback2(b, s)}
    );
    thread::spawn(move || {
        sender.register_publication(String::from("127.0.0.1"), 1833);

        let buf : [u8; 4] = [12, 14, 15, 1];
        for _ in 1 .. 10 {
            sender.send(&buf).expect("Failed to send buffer");
            sender.send_str("Hello Receiver".to_owned()).expect("Failed To Send String");
            thread::sleep(Duration::from_millis(250));
        }

    });
}

fn callback(bytes : &[u8], sock : &UdpComms) {
    // println!("Receiver Received {:?}", bytes);

    let str = String::from_utf8_lossy(bytes);

    println!("Receiver Recv Str = {}", str);

    if !str.is_empty() {
        sock.send_str("Hello Sender".to_owned()).expect("Failed to send string");
    }
}

fn callback2(bytes : &[u8], _ :&UdpComms) {
    println!("Sender Received {:?}", bytes);
}