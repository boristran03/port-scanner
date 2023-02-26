use std::env;
use std::collections::HashMap;
use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::time;

//second
const TIMEOUT: u64 = 500;

fn main() {
    let args: Vec<String> = env::args().collect();
    let hostname = &args[1];
    let start_port: i32 = args[2].parse().unwrap();
    let end_port: i32 = args[3].parse().unwrap();

    let mut port_opened: HashMap<String, Vec<i32>> = HashMap::new();
    let port_list = start_port..=end_port;

    for port in port_list {
        for target in resolve_domain(hostname, &port) {
            if scan_port(&target, port).is_ok() {
                let target_ip = Box::new(target.to_string());
                if!(port_opened.contains_key(target_ip.as_str())) {
                    port_opened.insert(target_ip.to_string(), Vec::new());
                }
                let ports = port_opened.get_mut(target_ip.as_str()).unwrap();
                ports.push(port);
            }
        }
    }

    println!("{:#?}", port_opened);
}

fn resolve_domain(hostname: &String, port: &i32) -> Vec<SocketAddr> {
    let target = format!("{}:{}", hostname.trim(), port);
    return target
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
}

// return port number when is opening or error string
fn scan_port(sockaddr: &SocketAddr, port: i32) -> Result<i32, String> {
    
    let stream = TcpStream::connect_timeout(sockaddr, time::Duration::from_millis(TIMEOUT));
    match stream {
        Ok(_) => return Ok(port),
        Err(error) => return Err(error.to_string()),
    }
}
