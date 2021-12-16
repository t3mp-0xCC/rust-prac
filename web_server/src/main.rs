use std::{
    env,
    str,
    path::Path,
    string,
    fs::File,
    net::{TcpListener, TcpStream}, 
    io::{Read, prelude, Write}, fmt::format,
};

fn main() {
    /*TODO: parse args and change port*/
    /*let args: Vec<String> = env::args().collect();
    let port = match args[1].as_str() {
        "-p" => args[2].as_str(),
        _ => "8080",
    };*/

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("[+] HTTP Server Started !");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut html = File::open("./index.html").expect("./index.html does not exist");

        let mut contents = String::new();
        html.read_to_string(&mut contents).unwrap();

        let responce = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(responce.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Responce: {}", responce);
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut html = File::open("./404.html").expect("./404.html does not exist !");

        let mut contents = String::new();
        html.read_to_string(&mut contents).unwrap();

        let responce = format!("{}{}", status_line, contents);

        stream.write(responce.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Responce: {}", responce);
    }
}
