use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

#[derive(Debug)]
struct Variable {
    key: String,
    val: String,
}

impl TryFrom<String> for Variable {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let results: Vec<_> = value.split('=').collect();
        if results.len() < 2 {
            Err("Input must have the format KEY=VAL")
        } else {
            Ok(Self {
                key: results[0].to_string(),
                val: results[1].to_string(),
            })
        }
    }
}

fn main() -> io::Result<()> {
    let filepath = env::args().nth(1);
    if filepath.is_none() {
        eprintln!("usage ento <filename>");
        process::exit(1);
    }

    let file = File::open(filepath.unwrap())?;
    let reader = BufReader::new(file);
    let mut variables: Vec<Variable> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let variable: Result<Variable, &str> = Variable::try_from(line);
        if variable.is_err() {
            continue;
        }
        let variable = variable.unwrap();
        env::set_var(&variable.key, &variable.val);
        variables.push(variable);
    }

    if variables.is_empty() {
        eprintln!("couldn't find any environment variables");
        process::exit(1);
    }

    println!("found and set the following environment variables");
    for variable in variables.iter() {
        println!("{}={}", variable.key, variable.val);
    }

    Ok(())
}
