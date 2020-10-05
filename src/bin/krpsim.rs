use std::env;
use std::fs::File;
use std::io::prelude::*;

use common::parser::parse;

const MAX_FILE_SIZE: u64 = 10000;

fn help(name: &str) {
    eprintln!("usage: {} file delay", name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help(&args[0]);
        return;
    }
    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {}", error);
            return;
        }
    };

    let _delay = match args[2].parse::<f32>() {
        Ok(nb) => nb,
        Err(error) => {
            eprintln!("Error reading delay: {}", error);
            return;
        }
    };

    let mut handle = file.take(MAX_FILE_SIZE);
    let mut buffer = String::new();
    if let Err(error) = handle.read_to_string(&mut buffer) {
        eprintln!("Error reading file: {}", error);
        return;
    }

    let krp = parse(&buffer[..]);
    if let Err(err) = krp {
        eprintln!("{}", err);
        return;
    }
    krp.unwrap().print_state();
}
