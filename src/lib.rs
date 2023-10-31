use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Variable {
    pub key: String,
    pub val: String,
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

pub fn from_file(path: String) -> Result<Vec<Variable>, &'static str> {
    let file = File::open(path);
    if file.is_err() {
        return Err("Couldn't open file");
    }

    let reader = BufReader::new(file.unwrap());
    let mut variables: Vec<Variable> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| "".to_string());
        let variable: Result<Variable, &str> = Variable::try_from(line);
        if variable.is_err() {
            continue;
        }
        let variable = variable.unwrap();
        env::set_var(&variable.key, &variable.val);
        variables.push(variable);
    }

    if variables.is_empty() {
        Err("Couldn't find any environment variables")
    } else {
        Ok(variables)
    }
}
