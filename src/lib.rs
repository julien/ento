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

impl From<&Variable> for String {
    fn from(value: &Variable) -> Self {
        format!("{}={}", value.key, value.val)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_variable_try_from_ok() {
        let input = "FOO=BAR";
        let var = Variable::try_from(input.to_string()).unwrap();
        assert_eq!(&var.key, "FOO");
        assert_eq!(&var.val, "BAR");
    }

    #[test]
    pub fn test_variable_try_from_ko() {
        let input = "FOO";
        let var: Result<Variable, &str> = Variable::try_from(input.to_string());
        assert_eq!(var.is_err(), true);
        assert_eq!(var.err().unwrap(), "Input must have the format KEY=VAL");
    }

    #[test]
    pub fn test_from_file_ok() {
        let res: Result<Vec<Variable>, &str> = from_file(String::from("./env"));
        let vars = res.unwrap();
        assert_eq!(vars.is_empty(), false);
    }

    #[test]
    pub fn test_from_file_non_exisiting_file() {
        let res: Result<Vec<Variable>, &str> = from_file(String::from("./nopes"));
        assert_eq!(res.is_err(), true);
        assert_eq!(res.err().unwrap(), "Couldn't open file");
    }

    #[test]
    pub fn test_from_file_no_variables() {
        let res: Result<Vec<Variable>, &str> = from_file(String::from("./empty"));
        assert_eq!(res.is_err(), true);
        assert_eq!(
            res.err().unwrap(),
            "Couldn't find any environment variables"
        );
    }

    #[test]
    pub fn test_from_variable_ok() {
        let var = &Variable {
            key: String::from("FOO"),
            val: String::from("BAR"),
        };
        let s: String = var.into();
        assert_eq!(s, "FOO=BAR");
    }
}
