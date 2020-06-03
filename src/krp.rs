#[derive(Debug)]
pub struct Process<'a> {
    pub name: &'a str,
    pub requirements: Vec<(&'a str, i32)>,
    pub results: Vec<(&'a str, i32)>,
    pub nb_cycle: i32,
}

#[derive(Debug)]
pub struct Krp<'a> {
    pub stock: Vec<(&'a str, i32)>,
    pub processes: Vec<Process<'a>>,
    pub optimize: Vec<&'a str>,
}
