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
        let Some((code, id)) = line.split_once("|") else {
            continue;
        };
        workers.insert(code.trim().to_string(), id.trim().to_string());
    }
    Ok(workers)
}


fn load_inside() -> io::Result<HashSet<String>> {
    let mut inside = HashSet::new();
    if !Path::new(RECORDS_FILE).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No se ha encontrado el archivo de registro"));
    }

    let content = fs::read_to_string(RECORDS_FILE)?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    for line in content.lines() {
        let fields: Vec<&str> = line.split("|").collect();
        
        if fields.len() != 3 {
            continue;
        }

    let id = fields[0];
    let action = fields[1];
    let timestamp = fields[2];

    if !timestamp.starts_with(&today) {
        continue;
    }

    match action {
        "E" => {
            inside.insert(id.to_string());
        }
        "S" => {
            inside.remove(id);
        }
        _ => {}
    }
    }
  return Ok(inside);
}

fn main() {
    return;
}



 
