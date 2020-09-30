use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::cmp::Ordering;

use common::krp::{Krp, Process};
use common::parser::{alpha, number, parse};
use nom::{bytes::complete::tag, sequence::tuple, IResult};

const MAX_FILE_SIZE: u64 = 10000;

fn parse_process(input: &str) -> IResult<&str, (i32, String)> {
    let (o, (p, _, q)) = tuple((number, tag(":"), alpha))(input)?;

    Ok((o, (p, q)))
}

fn parse_trace(trace: &str) -> IResult<&str, Vec<(i32, String)>> {
    let mut res: Vec<(i32, String)> = Vec::new();

    for line in trace.lines() {
        let (_, value) = parse_process(line)?;
        res.push(value);
    }
    res.sort_by(|(a, _), (b, _)| b.cmp(a));

    Ok((trace, res))
}

fn verifier(mut krp: Krp, mut walk: Vec<(i32, String)>) -> Option<()> {
    let mut active: VecDeque<(i32, &Process)> = VecDeque::new();
    while let Some((start, pname)) = walk.pop() {
        let process = krp.processes.get(&pname)?;
        'inner: while let Some((end, aproc)) = active.front() {
            if start < *end {
                break 'inner;
            }
            for (name, qty) in aproc.results.iter() {
                let stock = krp.stock.get_mut(name);
                match stock {
                    Some(x) => { *x += *qty },
                    None => { krp.stock.insert(name.clone(), *qty); }
                }
            }
            active.pop_front();
        }
        for (name, qty) in process.requirements.iter() {
            let x = krp.stock.get_mut(name)?;
            match (*x).cmp(qty) {
                Ordering::Less => return None,
                Ordering::Equal => { krp.stock.remove(name); },
                Ordering::Greater => *x -= *qty
            }
        }
        active.push_back((start + process.nb_cycle, process));
    }
    while let Some((_, aproc)) = active.front() {
        for (name, qty) in aproc.results.iter() {
            let stock = krp.stock.get_mut(name);
            match stock {
                Some(x) => { *x += *qty },
                None => { krp.stock.insert(name.clone(), *qty); }
            }
        }
        active.pop_front();
    }
    Some(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} file trace", args[0]);
        return;
    }
    let file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening config file: {}", error);
            return;
        }
    };

    let mut handle = file.take(MAX_FILE_SIZE);
    let mut buffer = String::new();
    if let Err(error) = handle.read_to_string(&mut buffer) {
        eprintln!("Error reading config file: {}", error);
        return;
    }

    let krp = parse(&buffer[..]);
    if let Err(error) = krp {
        eprint!("{}", error);
        return;
    }

    let file = match File::open(&args[2]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening trace file: {}", error);
            return;
        }
    };

    let mut handle = file.take(MAX_FILE_SIZE);
    let mut trace = String::new();
    if let Err(error) = handle.read_to_string(&mut trace) {
        eprintln!("Error reading config file: {}", error);
        return;
    }

    let walk = match parse_trace(&trace[..]) {
        Ok((_, v)) => v,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    if verifier(krp.unwrap(), walk) == None {
        println!("KO");
    } else {
        println!("OK");
    }
}
