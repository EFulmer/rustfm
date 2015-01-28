use std::io::net::tcp::TcpStream;

static API_ROOT : &'static str = "http://ws.audioscrobbler.com:80";

fn main() {
    let mut socket = TcpStream::connect(API_ROOT);

    socket.write(b"GET /2.0 HTTP/1.0\n\n");

    let response = socket.read_to_end();

    println!("{:?}", response);
}
