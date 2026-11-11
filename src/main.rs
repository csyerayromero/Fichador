use chrono::Local;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

const WORKERS_FILE: &str = "trabajadores.txt";
const RECORDS_FILE: &str = "fichajes.txt";
const DATE_FORMAT: &str = "%Y-%m-%d, %Hh:%Mm%Ss";

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

    for line in content.lines() {
        let fields: Vec<&str> = line.split("|").collect();
        
        if fields.len() !=4 {
            continue;
        }

        let id = fields[0];
        let action = fields[1];


        match action {
            "E" => {
                inside.insert(id.to_string());
            }
            "S" => {
                inside.remove(id);
            }
            _ => {panic!("Failed to parse action in {} file", RECORDS_FILE);}
        }
    }
  return Ok(inside);
}

fn reg(id: &mut String, action: &str){
    let date = Local::now().format(DATE_FORMAT).to_string();
    let workers = load_workers();
    let worker = workers.expect("Failed to retrieve workers").get(id).expect("No worker found").to_string();
    let content = format!("{}|{}|{}|{}\n", id, action, date, worker);
    let mut file = OpenOptions::new()
        .append(true)
        .create(false)
        .open(RECORDS_FILE)
        .expect("No se pudo escribir en el archivo de registro");
    
    file.write_all(content.as_bytes()).expect("No se pudo escribir el archivo en el registro (2)");

}

fn reg_in(id: &mut String) {
    let action = "E";
    reg(id, action);
}

fn reg_out(id: &mut String) {
    let action = "S";
    reg(id, action);
}

fn register(scan: &mut String) -> bool {
    let already_inside = load_inside().unwrap();
    if ! already_inside.contains(scan) {
        reg_in(scan); 
        return false;
    }else {
        reg_out(scan);
        return true;
    }

}

fn main() {
    let stdin = io::stdin();
    let mut barcode = String::new();
    loop {
        match stdin.lock().read_line(&mut barcode) {
            Ok(0) => break,
            Ok(_) => {
                barcode = barcode.trim().to_string();
                let mut scan = String::from(&barcode);
                register(&mut scan);
                barcode = String::new();
                println!("Scanned: {}", barcode);
            },
            Err(e) => eprintln!("Input error: {}", e),
        }
    }
}



 
