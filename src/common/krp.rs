use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Process<'a> {
    pub name: &'a str,
    pub requirements: Vec<(&'a str, i32)>,
    pub results: Vec<(&'a str, i32)>,
    pub nb_cycle: i32,
}

#[derive(Debug)]
pub struct Krp<'a> {
    pub stock: HashMap<&'a str, i32>,
    pub processes: HashMap<&'a str, Process<'a>>,
    pub optimize: Vec<&'a str>,
}

impl<'a> Krp<'a> {
    pub fn consume(&mut self, process: &'a Process) -> Option<()> {
        for (name, qty) in process.requirements.iter() {
            let x = self.stock.get_mut(name)?;
            match (*x).cmp(qty) {
                Ordering::Less => return None,
                Ordering::Equal => { self.stock.remove(name); },
                Ordering::Greater => *x -= *qty
            }
        }
        Some(())
    }

    pub fn produce(&mut self, process: &'a Process) -> Option<()> {
        for (name, qty) in process.requirements.iter() {
            let stock = self.stock.get_mut(name);
            match stock {
                Some(x) => { *x += *qty },
                None => { self.stock.insert(name, *qty); }
            }
        }
        Some(())
    }
}