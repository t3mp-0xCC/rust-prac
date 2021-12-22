use std::{
    thread,
    env,
    str,
    path::Path,
    string,
    fs::File,
    net::{TcpListener, TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr}, 
    io::{Read, prelude, Write}, fmt::format,
};
use clap::{App, Arg, SubCommand};

fn main() {
    /*TODO: parse args and change port*/
    /*TODO: make log file*/

    let args: Vec<String> = env::args().collect();
    let port: u16 = match args[1].as_str() {
        "-p" => args[2].parse::<u16>().unwrap(),
        _ => "8080".parse::<u16>().unwrap(),
    };

    let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    socket.set_port(port);

    let listener = TcpListener::bind(socket).unwrap();
    println!("[+] HTTP Server Started !");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./404.html")
    };
        
    let mut html = File::open(filename).unwrap();
    let mut contents = String::new();
    html.read_to_string(&mut contents).unwrap();

    let responce = format!("{}{}", status_line, contents);

    stream.write(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Responce: {}", responce);
}
