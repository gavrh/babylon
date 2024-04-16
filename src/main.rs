use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::Read;
use std::io::prelude::Write;
use std::fs;
use std::thread;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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
    println!("Req: {}", String::from_utf8_lossy(&buffer[..]));

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let content = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
}
