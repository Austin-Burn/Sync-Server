use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::io;
use std::collections::HashMap;
use dashmap::DashMap;
use threadpool::ThreadPool;
  
pub fn init_server(type: bool, ) -> io::Result<()> {

}


fn spawn_delegator_thread() -> io::Result<()> {
    make_new_thread()
}



pub struct UDPServer {
    task_max_backlog: u8,
    chunksize: u64,
    packet_size: u32,
    target_ip: Option<String>,
    socket: UdpSocket,
    upload_download: bool,
    active_ports: Vec<u16>,
    //1 is data, 2 is add thread, 3 is remove thread, 4 is end connection
    commands: HashMap<String, Box<dyn Fn() -> io::Result<()> + Send + Sync>>,
    worker_threads: DashMap<String, Thread>,
    delegator_thread: Thread,
    data_buffer_one: Vec<u8>,
    data_buffer_two: Vec<u8>,
    data_buffer_three: Vec<u8>,
    thread_queues: DashMap<String, Vec<u8>>,
}

pub struct load_buffer {
    const type: u8 = 1,
    data_pointer: u8,
    buff_size: u8,
}

impl UDPServer {

    fn spawn_worker_thread() -> io::Result<()> {
        make_new_thread()
        struct thread_queue {
            thread_id: String,
            buffer_switch: bool,
            thread_que_one: Vec<u8>,
            thread_que_two: Vec<u8>,

        }
        
        self.thread_queues.insert(thread_id, thread_queue {
            thread_id: thread_id,
            buffer_switch: false,
            thread_que_one: Vec::new(),
            thread_que_two: Vec::new(),
        }   );
    
    }

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

