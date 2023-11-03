use ento::*;
use std::{env, io, process};

fn main() -> io::Result<()> {
    let path = env::args().nth(1);
    if path.is_none() {
        eprintln!("Pass a file name as the first argument");
        process::exit(1);
    }

    let variables: Result<Vec<Variable>, &str> = from_file(path.unwrap());
    if variables.is_err() {
        eprintln!("Error: {:?}", variables.unwrap());
        process::exit(1);
    }
    let variables = variables.unwrap();

    println!("Found and set the following environment variables");
    for variable in variables.iter() {
        let s: String = variable.into();
        println!("{}", s);
    }

    Ok(())
}
