use std::net::{SocketAddr, TcpListener};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::fs::read;

fn main() {
    // Listen on a port
    let port: u16 = 9999;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address).unwrap();

    for mut stream in listener.incoming().flatten() {
        // Request
        let mut rdr = BufReader::new(&mut stream);
        // First Line
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() { break; }
                    print!("{l}");
                }
                // Response
                let mut p = PathBuf::new();
                p.push("htdocs");
                // Load (NOT SECURE)
                p.push(resource.trim_start_matches("/"));
                if resource.ends_with('/') { p.push("index.html"); }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                stream.write_all(&read(p).unwrap()).unwrap();
            }
            _ => todo!()
        }
        
    }
}
