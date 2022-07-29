use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum EnvError {
    InvalidUnicode(String),
    NotFound(String),
}

impl Display for EnvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::InvalidUnicode(name) => {
                writeln!(f, "invalid unicode on the value of {}", name)
            }
            EnvError::NotFound(name) => writeln!(f, "environment variable {} not found", name),
        }
    }
}

impl Error for EnvError {}

pub fn get(name: String) -> Result<String, Box<dyn Error>> {
    match env::var_os(&name) {
        Some(osenv) => {
            if let Ok(osstrenv) = osenv.into_string() {
                Ok(osstrenv)
            } else {
                Err(Box::new(EnvError::InvalidUnicode(name)))
            }
        }
        None => Err(Box::new(EnvError::NotFound(name))),
    }
}

pub fn get_path() -> Result<Vec<String>, Box<dyn Error>> {
    let mut result :Vec<String> = Vec::new();
    for p in get(String::from("PATH"))?.split(":") {
        result.push(String::from(p));
    }
    Ok(result)
}

pub fn get_all() -> Result<HashMap<String,String>, Box<dyn Error>> {
    let mut result :HashMap<String, String> = HashMap::new();
    for (key, val) in env::vars_os() {
        if let Ok(envkey) = key.into_string() {
            if let Ok(envval) = val.into_string() {
                result.insert(envkey, envval);
            } else {
                return Err(Box::new(EnvError::InvalidUnicode(envkey)));
            }
        } else {
            return Err(Box::new(EnvError::InvalidUnicode(String::from("<key>"))));
        }
    }

    Ok(result)
}
