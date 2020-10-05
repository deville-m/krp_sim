use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub requirements: Vec<(String, i32)>,
    pub results: Vec<(String, i32)>,
    pub nb_cycle: i32,
}

#[derive(Debug)]
pub struct Krp {
    pub stock: HashMap<String, i32>,
    pub processes: HashMap<String, Process>,
    pub optimize: Vec<String>,
}

impl Krp {
    pub fn consume(&mut self, process: &Process) -> Option<()> {
        for (name, qty) in process.requirements.iter() {
            let x = self.stock.get_mut(name)?;
            match (*x).cmp(qty) {
                Ordering::Less => return None,
                Ordering::Equal => {
                    self.stock.remove(name);
                }
                Ordering::Greater => *x -= *qty,
            }
        }
        Some(())
    }

    pub fn produce(&mut self, process: &Process) -> Option<()> {
        for (name, qty) in process.results.iter() {
            let stock = self.stock.get_mut(name);
            match stock {
                Some(x) => *x += *qty,
                None => {
                    self.stock.insert(name.clone(), *qty);
                }
            }
        }
        Some(())
    }

    pub fn print_state(&self) {
        println!(
            "# {} items in stock, {} processes, {} to optimize",
            self.stock.len(),
            self.processes.len(),
            self.optimize.len()
        );
        println!("# === Stocks:");
        for stock in self.stock.iter() {
            println!("# {}: {}", stock.0, stock.1);
        }

        println!("# === Processes:");
        for (pname, process) in self.processes.iter() {
            println!("# {}: {}", pname, process.nb_cycle);
        }

        println!("# === Optimize:");
        for optimize in self.optimize.iter() {
            println!("# {}", optimize);
        }
    }
}
