use std::collections::HashMap;
use std::env;
use std::error::Error;

mod env_unicode;
mod path;
mod subzsh;

pub enum EnvQuery {
    Single(String),
    Multi(Vec<String>),
}

pub enum EnvResult {
    Single(String),
    Multi(HashMap<String, String>),
}

pub enum EnvInput {
    KeyVal(String, String),
    Map(HashMap<String, String>),
}

pub trait SubShell {
    fn exec(&self);
    fn get_all_env(&self) -> Result<EnvResult, Box<dyn Error>>;
    fn get_env(&self, q: EnvQuery) -> Result<EnvResult, Box<dyn Error>>;
    fn set_env(&mut self, input: EnvInput) -> Result<(), Box<dyn Error>>;
}

pub fn prepare_zsh() -> Result<Box<dyn SubShell>, Box<dyn Error>> {
    subzsh::prepare()
}
