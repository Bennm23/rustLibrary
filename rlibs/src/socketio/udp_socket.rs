use std::{fmt::Error, net::UdpSocket, thread};
pub type MsgCallback = fn(&[u8], &UdpComms);

pub struct UdpComms {
    socket   : UdpSocket,
    addr     : String,
    port     : i32,
    callback : MsgCallback,
    publications : Vec<String>,
}

impl UdpComms {

    pub fn new(addr : String, port : i32, callback : MsgCallback) -> Self {

        let sock = UdpSocket::bind(format!("{}:{}", addr,port))
            .expect("Failed To Create UdpSocket");

        Self {
            socket: sock,
            addr,
            port,
            callback,
            publications : Vec::new(),
        }
    }

    pub fn start(&self) {

        let mut buf = [0; 1024];
        for _ in 1 .. 100 {
            let (amt, src) = self.socket.recv_from(&mut buf).expect("Socket Receive Failed");

            (self.callback)(&buf[..amt], self);

            println!("Received {} bytes from addr {}", amt, src);
        }

    }

    pub fn register_publication(&mut self, pub_addr : String, pub_port : i32) {
        self.publications.push(format!("{}:{}", pub_addr, pub_port));
    }

    pub fn send(&self, buf : &[u8]) -> Result<(), std::io::Error> {

        for addr in &self.publications {

            let _ = match self.socket.send_to(buf, addr) {
                Ok(bytes) => println!("Sent {} bytes to {}", bytes, addr),
                Err(e) => return Result::Err(e),
            };
        }

        Ok(())
    }

    pub fn send_str(&self, str : String) -> Result<(), std::io::Error> {
        self.send(str.as_bytes())
    }
    
}