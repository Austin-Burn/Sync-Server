use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io;

fn get_new_port() -> io::Result<u16> {
    // Try to bind to port 0, which tells the OS to assign an available port
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    // Get the actual port that was assigned
    let addr = socket.local_addr()?;
    Ok(addr.port())
}

pub struct UDPServer {
    socket: UdpSocket,
}

impl UDPServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }

    pub fn run(&self) -> io::Result<()> {
        let mut buf = [0; 1024];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    println!("Received {} bytes from {}", size, addr);
                    self.socket.send_to(&buf[..size], addr)?;
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                }
            }
        }
    }
}

