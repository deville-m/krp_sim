use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use common::krp::{Krp, Process};
use common::parser::{alpha, number, parse};
use nom::{bytes::complete::tag, sequence::tuple, IResult};

const MAX_FILE_SIZE: u64 = 10000;

fn parse_process(input: &str) -> IResult<&str, (i32, String)> {
    let (o, (p, _, q)) = tuple((number, tag(":"), alpha))(input)?;

    Ok((o, (p, q)))
}

fn comment(input: &str) -> IResult<&str, &str> {
    tag("#")(input)
}

fn parse_trace(trace: &str) -> IResult<&str, Vec<(i32, String)>> {
    let mut res: Vec<(i32, String)> = Vec::new();

    for line in trace.lines() {
        if comment(line).is_ok() {
            continue;
        }
        let (_, value) = parse_process(line)?;
        res.push(value);
    }
    res.sort_by(|(a, _), (b, _)| b.cmp(a));

    Ok((trace, res))
}

fn verifier(mut krp: Krp, mut walk: Vec<(i32, String)>) -> Result<(Krp, i32), (i32, String, Krp)> {
    let mut active: VecDeque<(i32, &Process)> = VecDeque::new();
    let mut last_cycle: i32 = 0;
    while let Some((start, pname)) = walk.pop() {
        let process = krp.processes.get(&pname);
        if process.is_none() {
            return Err((start, pname, krp));
        }
        let process = process.unwrap();
        'inner: while let Some((end, aproc)) = active.front() {
            if start < *end {
                break 'inner;
            }
            last_cycle = *end;
            for (name, qty) in aproc.results.iter() {
                let stock = krp.stock.get_mut(name);
                match stock {
                    Some(x) => *x += *qty,
                    None => {
                        krp.stock.insert(name.clone(), *qty);
                    }
                }
            }
            active.pop_front();
        }
        for (name, qty) in process.requirements.iter() {
            let x = krp.stock.get_mut(name);
            if x.is_none() {
                return Err((start, pname, krp));
            }
            let x = x.unwrap();
            match (*x).cmp(qty) {
                Ordering::Less => return Err((start, pname, krp)),
                Ordering::Equal => {
                    krp.stock.remove(name);
                }
                Ordering::Greater => *x -= *qty,
            }
        }
        active.push_back((start + process.nb_cycle, process));
    }
    while let Some((end, aproc)) = active.front() {
        last_cycle = *end;
        for (name, qty) in aproc.results.iter() {
            let stock = krp.stock.get_mut(name);
            match stock {
                Some(x) => *x += *qty,
                None => {
                    krp.stock.insert(name.clone(), *qty);
                }
            }
        }
        active.pop_front();
    }
    Ok((krp, last_cycle))
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
        eprintln!("{}", error);
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

    match verifier(krp.unwrap(), walk) {
        Ok((krp, end)) => {
            krp.print_state();
            println!("OK at cycle {}", end)
        }
        Err((cycle, pname, krp)) => {
            krp.print_state();
            println!("KO at cycle {}: {}", cycle, pname)
        }
    }
}
