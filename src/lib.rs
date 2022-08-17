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
mod error;
pub mod profile;

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
    fn apply_environment(&self, profile : &dyn Profile) -> Result<(), Box<dyn Error>>;
}

pub trait Profile {
    fn exist(&self) -> Result<&dyn Profile, Box<dyn Error>>;
    fn set_profile_name(&mut self, name : String) -> &mut dyn Profile;
    fn profile_path(&self) -> Result<String, Box<dyn Error>>;
    fn list(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn apply(&self) -> Result<(), Box<dyn Error>>;
}

pub fn prepare_zsh() -> Result<Box<dyn SubShell>, Box<dyn Error>> {
    subzsh::prepare()
}