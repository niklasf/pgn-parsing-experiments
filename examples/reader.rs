extern crate pgn_reader;

use std::fs::File;
use std::env;
use std::str;
use pgn_reader::RawHeader;
use pgn_reader::reader::{PgnReader, Visitor};

struct MyVisitor;

impl Visitor for MyVisitor {
    type Result = ();

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        println!("{:?}: {:?}", String::from_utf8_lossy(key), value);
    }

    fn end_game(&mut self) { }
}

fn main() {
    for arg in env::args().skip(1) {
        let file = File::open(&arg).expect("fopen");
        let mut reader = PgnReader::new(file);
        reader.read_game(&mut MyVisitor);
    }
}