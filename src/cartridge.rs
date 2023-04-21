use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    pub data: [u8; 3584]
}

impl Cartridge {
    pub fn load(filename: &str) -> Self {

        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [0u8; 3584];
        f.read(&mut buffer).unwrap();
	return Cartridge {
	    data: buffer,
	}
    }
}
