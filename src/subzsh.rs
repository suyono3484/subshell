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

use crate::{path, env_unicode, EnvInput, EnvQuery, EnvResult, SubShell, Profile};
use std::collections::HashMap;
use std::{env, fs};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::path::Path;
use std::io::ErrorKind;
use exec;

#[derive(Debug)]
pub struct SubZsh {
    path: String,
    env: HashMap<String, String>,
}

impl SubShell for SubZsh {
    fn exec(&self) -> Result<(), Box<dyn Error>>{
        let err = exec::Command::new(&self.path).arg("-l").exec();
        Err(Box::new(err))
    }

    fn get_all_env(&self) -> Result<EnvResult, Box<dyn Error>> {
        Ok(EnvResult::Multi(self.env.clone()))
    }

    fn get_env(&self, q: EnvQuery) -> Result<EnvResult, Box<dyn Error>> {
        match q {
            EnvQuery::Single(name) => {
                let str_name:&str = &name;
                match self.env.get(str_name) {
                    Some(x) => Ok(EnvResult::Single(String::from(x))),
                    None => Err(Box::new(env_unicode::EnvError::NotFound(name))),
                }
            },
            EnvQuery::Multi(names) => {
                let mut result :HashMap<String, String> = HashMap::new();
                for e in names {
                    let ec = e.clone();
                    match self.get_env(EnvQuery::Single(e)) {
                        Ok(sub_result) => {
                            match sub_result {
                                EnvResult::Single(x) => {
                                    result.insert(ec, x);
                                },
                                EnvResult::Multi(_e) => panic!("invalid state"),
                            }
                        }
                        Err(_err) => continue,
                    }
                }
                if result.is_empty() {
                    return Err(Box::new(env_unicode::EnvError::NotFound(String::from("<multiple keys>"))));
                }
                Ok(EnvResult::Multi(result))
            },
        }
    }

    fn get_home_env(&self) -> Result<String, Box<dyn Error>> {
        match self.get_env(EnvQuery::Single(String::from("HOME")))? {
            EnvResult::Single(h) => {
                return Ok(h);
            }
            EnvResult::Multi(_m) => panic!("invalid state"),
        }
    }

    fn set_env(&mut self, input: EnvInput) -> Result<(), Box<dyn Error>> {
        match input {
            EnvInput::KeyVal(key, val) => {
                self.env.insert(key, val);
                Ok(())
            },
            EnvInput::Map(m) => {
                for (key, val) in m {
                    self.set_env(EnvInput::KeyVal(key.clone(), val.clone()))?;
                }
                Ok(())
            },
        }
    }

    fn apply_environment(&self, profile : &dyn Profile) -> Result<(), Box<dyn Error>> {
        for (key, val) in &self.env {
            env::set_var(&key, &val);
        }

        let home = self.get_home_env()?;

        for f in [".zshenv", ".zprofile", ".zshrc", ".zlogin", ".zlogout"] {
            let profile_dir = format!("{}/{}", profile.profile_path()?, f);
            match fs::metadata(Path::new(&profile_dir)) {
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        let home_file = format!("{}/{}", &home, f);
                        if let Ok(_meta) = fs::metadata(Path::new(&home_file)) {
                            fs::copy(&home_file, &profile_dir)?;
                        }
                    },
                    _other_error => return Err(Box::new(e)),
                },
                _other_ok=> {},
            }
        }

        env::set_var("ZDOTDIR", profile.profile_path()?);
        Ok(())
    }
}

impl SubZsh {
    fn populate_env_hashmap(&mut self) -> Result<(), Box<dyn Error>> {
        self.env = env_unicode::get_all()?;
        Ok(())
    }
}

pub fn prepare_zsh() -> Result<SubZsh, Box<dyn Error>> {
    let mut zsh = SubZsh {
        path: path::find_in_path(String::from("zsh"))?,
        env: HashMap::new(),
    };
    zsh.populate_env_hashmap()?;
    println!("data: {:?}", zsh);

    Ok(zsh)
}

pub fn prepare() -> Result<Box<dyn SubShell>, Box<dyn Error>> {
    match prepare_zsh() {
        Ok(zsh) => Ok(Box::new(zsh)),
        Err(e) => Err(e),
    }
}