use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser;
use parser::parse;

mod krp;

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
    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {}", error);
            return;
        }
    };

    let delay = match args[2].parse::<f32>() {
        Ok(nb) => nb,
        Err(_) => {
            println!("Problem reading delay");
            return;
        }
    };

    let mut buffer = String::new();
    if file.read_to_string(&mut buffer).is_err() {
        eprintln!("Error reading file");
        return;
    }

    let krp = parse(&buffer[..]);
    if let Err(err) = krp {
        eprint!("{}", err);
        return;
    }
    println!("{:#?}", krp);
}
