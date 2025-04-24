use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io;
use std::collections::HashMap;
use dashmap::DashMap;
use threadpool::ThreadPool;
  

fn spawn_delegator_thread() -> io::Result<()> {
    make_new_thread()
}

pub struct UDPServer {
    chunksize: u64,
    packet_size: u32,
    target_ip: Option<String>,
    socket: UdpSocket,
    upload_download: bool,
    active_ports: Vec<u16>,
    //1 is data, 2 is add thread, 3 is remove thread, 4 is end connection
    commands: HashMap<String, fn() -> io::Result<()>>,
    packets: DashMap<String, Vec<u8>>,
    worker_threads: DashMap<String, Thread>,
    delegator_thread: Thread,
    buffer_one: Vec<u8>,
    buffer_two: Vec<u8>,
    buffer_three: Vec<u8>,
    
}

impl UDPServer {
    pub fn new(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket, upload_download: false, active_ports: Vec::new(), commands: HashMap::new(), packets: DashMap::new() })
    }

    fn make_new_thread() -> io::Result<()> {
        let pool = ThreadPool::new(1);
        pool.execute(|| {
            println!("Hello, world!");
        });
        Ok(())
    }
    
    fn get_new_port() -> io::Result<u16> {
        // Try to bind to port 0, which tells the OS to assign an available port
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        // Get the actual port that was assigned
        let addr = socket.local_addr()?;
        Ok(addr.port())
    }
    
    fn check_system_resources() -> io::Result<()> {
        let system_info = System::new_all();
        let cpu_usage = system_info.cpu_usage();
        let memory_usage = system_info.memory_usage();
        Ok(())
    }
    
    fn spawn_outbound_thread_manager() -> io::Result<()> {q
        make_new_thread()
    }
    
    fn spawn_inbound_thread_manager() -> io::Result<()> {
        make_new_thread()
    }
    
    fn spawn_thread_worker() -> io::Result<()> {
        make_new_thread()
        while true {
            let packet = buffer_one.pop_front();
            if packet.is_some() {
                let packet = packet.unwrap();
                let packet_data = packet.data;
            }
        }
    }   
    
    fn spawn_delegator_thread() -> io::Result<()> {
        make_new_thread()
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

