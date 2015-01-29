extern crate hyper;

use self::hyper::Client;

use std::old_io::{BufferedReader, File};

static API_ROOT: &'static str = "http://ws.audioscrobbler.com:80";

// TODO make this into a lastfm struct, containing some sort of ocnnection object & both keys

pub struct Keys {
    api: String,
    secret: String,
}

impl Keys {
    pub fn new(key_file: &Path) -> Keys {
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
