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

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::io::ErrorKind;
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
    fn exec(&self) -> Result<(), Box<dyn Error>>;
    fn get_all_env(&self) -> Result<EnvResult, Box<dyn Error>>;
    fn get_env(&self, q: EnvQuery) -> Result<EnvResult, Box<dyn Error>>;
    fn get_home_env(&self) -> Result<String, Box<dyn Error>>;
    fn set_env(&mut self, input: EnvInput) -> Result<(), Box<dyn Error>>;
    fn apply_environment(&self, profile_path :String) -> Result<(), Box<dyn Error>>;
}

pub fn prepare_zsh() -> Result<Box<dyn SubShell>, Box<dyn Error>> {
    subzsh::prepare()
}

pub fn apply_profile(s :&dyn SubShell, profile_name :String) -> Result<(), Box<dyn Error>> {
    let home = s.get_home_env()?;
    let base_dir = format!("{}/.subshell", home);
    let profile_dir = format!("{}/{}", &base_dir, profile_name);
    match fs::metadata(Path::new(&base_dir)) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                fs::create_dir(Path::new(&base_dir))?;
                fs::create_dir(Path::new(&profile_dir))?;
            },
            _other_error => {
                return Err(Box::new(e));
            },
        },
        _ok_result=> {},
    }

    fs::metadata(Path::new(&profile_dir))?;
    s.apply_environment(profile_dir)?;
    Ok(())
}

pub fn list_profiles(s :&dyn SubShell) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result :Vec<String> = Vec::new();


    Ok(result)
}