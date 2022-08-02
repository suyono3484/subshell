/*
 *    Copyright 2022 Suyono
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 *
 */

use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EnvError {
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
