use std::fs::File;
use std::collections::HashMap;
use std::io::{self, Read};
use std::default::Default;
use std::string;

#[derive(Debug)]
pub struct Parser {
    file: Option<File>,
    variables: HashMap<String, String>, 
}

impl TryFrom<File> for Parser {
    fn try_from(mut file: File) -> Result<Self, io::Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(Parser {
            file: Some(file),
            variables: Self::parse_string(&buf),
        })
    }
}

impl From<String> for Parser {
    fn from(value: String) -> Self {
        Parser {
            file: None,
            variables: Self::parse_string(&value)
        }
    }
}

impl Parser {
    pub fn load_file(filename: &str) -> Result<Self, io::Error> {
        let mut buffer = String::new();
        let mut file = File::open(filename)?;
        file.read_to_string(&mut buffer)?;
        
        Ok(Parser { file: Some(file), variables: Self::parse_string(&buffer) })
    }

    fn parse_string(content: &String) -> HashMap<String, String> {
        let mut lines: HashMap<String, String> = HashMap::new();
        let mut prefix = "";
        
        for line in content.lines() {
            if line.starts_with('#') {
                continue;
            } else if line.starts_with('[') {
                prefix = line.trim_matches(['[', ']']);
            } else if let Some((key, value)) = line.split_once('=') {
                let fullkey = if prefix.is_empty() {
                    key.to_string()
                } else {
                    prefix.to_string() + "." + key
                };
                lines.insert(fullkey, value.to_string());
            } else {
                continue;
            }
        }
        lines
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_load() {

    }
}