use std::io::{BufferedReader, File};
use std::io::net::tcp::TcpStream;

static API_ROOT: &'static str = "http://ws.audioscrobbler.com:80";

struct Keys {
    api: String,
    secret: String,
}

impl Keys {
    fn from_file() -> Keys {
        let mut file = match File::open(&Path::new("src/api.txt")) {
            Ok(f) => f,
            Err(e) => panic!("error encountered opening api keys file: {:?}", e),
        };

        let mut reader = BufferedReader::new(file);
        let mut a = reader.read_line().ok().expect("Failed to read API key from file");
        let mut s = reader.read_line().ok().expect("Failed to read secret key from file");

        a.pop(); // discarding newlines
        s.pop(); // ditto
        Keys { api: a, secret: s }
    }
}

fn main() {
    let mut socket = match TcpStream::connect(API_ROOT) {
        Ok(r) => r,
        Err(e) => panic!("error establishing connection to last.fm API server: {:?}", e),
    };

    socket.write(b"GET /2.0 HTTP/1.0\n\n");

    let response = socket.read_to_end();

    match response {
        Ok(t)    => println!("{}", String::from_utf8(t).unwrap()),
        Err(why) => println!("{:?}", why)
    };

}
