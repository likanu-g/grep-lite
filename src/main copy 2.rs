use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main(){
    let file = File::open("test").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        line.truncate(0);//重置line的大小

    }
} 