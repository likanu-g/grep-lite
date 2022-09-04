extern crate clap;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use regex::Regex;
use clap::{App,Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line), 
            None    => (),
        }
    }
}

fn main(){
    let args = App::new("grep-lite")
            .version("0.1.0")
            .about("search for patterns")
            .arg(
                Arg::with_name("pattern")
                .help("The patterns to search for")
                .takes_value(true)
                .required(true))
            .arg(
                Arg::with_name("input")
                .help("File to search for")
                .takes_value(true)
                .required(false))
            .get_matches();

   
    let pattern = args.value_of("pattern").unwrap();
    let re =  Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }

} 