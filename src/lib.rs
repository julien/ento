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
