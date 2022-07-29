use crate::{path, EnvInput, EnvQuery, EnvResult, SubShell};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SubZsh {
    path: String,
    env: HashMap<String, String>,
}

impl SubShell for SubZsh {
    fn exec(&self) {
        todo!()
    }

    fn get_all_env(&self) -> Result<EnvResult, Box<dyn Error>> {
        todo!()
    }

    fn get_env(&self, q: EnvQuery) -> Result<EnvResult, Box<dyn Error>> {
        todo!()
    }

    fn set_env(&mut self, input: EnvInput) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl SubZsh {
    fn populate_env_hashmap(&mut self) -> Result<(), Box<dyn Error>> {
        self.env = crate::env_unicode::get_all()?;
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
