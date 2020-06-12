use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

use common::krp::{Krp, Process};
use common::parser::{alpha, number, parse};
use nom::{bytes::complete::tag, sequence::tuple, IResult};

fn parse_process(input: &str) -> IResult<&str, (i32, &str)> {
    let (o, (p, _, q)) = tuple((number, tag(":"), alpha))(input)?;

    Ok((o, (p, q)))
}

fn parse_trace(trace: &str) -> IResult<&str, Vec<(i32, &str)>> {
    let mut res: Vec<(i32, &str)> = Vec::new();

    for line in trace.lines() {
        let (_, value) = parse_process(line)?;
        res.push(value);
    }
    res.sort_by(|(a, _), (b, _)| b.cmp(a));

    Ok((trace, res))
}

fn verifier<'a>(mut krp: Krp<'a>, mut walk: Vec<(i32, &str)>) -> Option<()> {
    let mut active: VecDeque<(i32, &'a Process)> = VecDeque::new();
    while let Some((start, pname)) = walk.pop() {
        let process = krp.processes.get(pname)?;
        'inner: while let Some((end, aproc)) = active.front() {
            if start < *end {
                break 'inner;
            }
            //krp.produce(*aproc)?;
            active.pop_front();
        }
        krp.consume(process)?;
        active.push_back((start + process.nb_cycle, process));
    }
    Some(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} file trace", args[0]);
        return;
    }
    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening config file: {}", error);
            return;
        }
    };

    let mut buffer = String::new();
    if file.read_to_string(&mut buffer).is_err() {
        eprintln!("Error reading config file");
        return;
    }

    let krp = match parse(&buffer[..]) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut file = match File::open(&args[2]) {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening trace file: {}", error);
            return;
        }
    };

    let mut trace = String::new();
    if file.read_to_string(&mut trace).is_err() {
        eprintln!("Error reading config file");
        return;
    }

    let walk = match parse_trace(&trace[..]) {
        Ok((_, v)) => v,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    if verifier(krp, walk) == None {
        println!("OK");
    } else {
        println!("KO");
    }
}
