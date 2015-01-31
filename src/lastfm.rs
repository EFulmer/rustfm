extern crate hyper;

use self::hyper::Client;
use std::old_io::{BufferedReader, File};

static API_ROOT: &'static str = "http://ws.audioscrobbler.com/2.0";

struct Keys {
    api: String,
    secret: String,
}

impl Keys {
    fn from_file(key_file: &Path) -> Keys {
        let file = match File::open(key_file) {
            Ok(f) => f,
            Err(e) => panic!("error encountered opening api keys file: {:?}", e),
        };

        let mut reader = BufferedReader::new(file);
        let mut a = reader.read_line().ok()
            .expect("Failed to read API key from file");
        let mut s = reader.read_line().ok()
            .expect("Failed to read secret key from file");

        a.pop(); // discarding newlines
        s.pop(); // ditto
        Keys { api: a, secret: s }
    }
}

pub mod artist {
    use super::hyper::Client;
    use super::Keys;
    use super::*;


    pub fn get_info(keys: Keys, artist: &str) -> String {
        let url = format!("{root}?method=artist.getinfo&artist={artist}&api_key={key}&format=json", 
                          root=super::API_ROOT, artist=artist, key=keys.api);
    
        let mut client = Client::new();
        let mut res = client.get(url.as_slice()).send().unwrap();
    
        res.read_to_string().unwrap()
    }

    pub fn get_similar(keys: Keys, artist: &str) -> String {
        let url = format!("{root}?method=artist.getsimilar&artist={artist}&api_key={key}&format=json", 
                          root=super::API_ROOT, artist=artist, key=keys.api);
    
        let mut client = Client::new();
        let mut res = client.get(url.as_slice()).send().unwrap();
    
        res.read_to_string().unwrap()
    }
}

#[test]
fn test_artist_get_info() {
    let keys = Keys::from_file(&Path::new("src/etc/keys.txt"));

    let info = artist::get_info(keys, "Oasis");
    println!("{}", info); // cargo test -- --nocapture to print out

    // sort of basic and dumb right now
    assert!(&info[].contains("Oasis"));
}

#[test]
fn test_artist_get_similar() {
    let keys = Keys::from_file(&Path::new("src/etc/keys.txt"));

    let info = artist::get_similar(keys, "Oasis");
    println!("{}", info);

    // sort of basic and dumb right now
    assert!(&info[].contains("Blur"));
}
