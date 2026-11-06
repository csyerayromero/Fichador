use chrono::Local;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

const WORKERS_FILE: &str = "trabajadores.txt";
const RECORDS_FILE: &str = "fichajes.txt";

fn load_workers() -> io::Result<HashMap<String, String>> {
    let content = fs::read_to_string(WORKERS_FILE)?;
    let mut workers = HashMap::new();

    for line in content.lines() {
        let Some((code, id)) = line.split_once('|') else {
            continue;
        };
        workers.insert(code.trim().to_string(), id.trim().to_string());
    }
    Ok(workers)
}


fn 

 
