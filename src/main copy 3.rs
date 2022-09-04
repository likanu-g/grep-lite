use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main(){
    let f = File::open("test").unwrap();

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let lin = line.unwrap();
        println!("{}({} bytes long)", lin, lin.len() as u64);
    }

} 