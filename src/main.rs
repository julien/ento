use std::{env, io, process};

fn main() -> io::Result<()> {
    let path = env::args().nth(1);
    if path.is_none() {
        eprintln!("usage: ento <filename>");
        process::exit(1);
    }

    let variables: Result<Vec<ento::Variable>, &str> = ento::from_file(path.unwrap());
    if variables.is_err() {
        eprintln!("error: {:?}", variables.unwrap());
        process::exit(1);
    }
    let variables = variables.unwrap();

    println!("found and set the following environment variables");
    for variable in variables.iter() {
        let s: String = variable.into();
        println!("{}", s);
    }

    Ok(())
}
