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

use std::borrow::Borrow;
use std::env;
use std::error::Error;
use subshell::{EnvQuery, SubShell, EnvResult};
use std::fs;
use std::path::Path;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
    let p = subshell::prepare_zsh();
    match p {
        Ok(q) => {
            apply_profile(q.borrow(), String::from("default"))?;
            q.exec();
        }
        Err(e) => {
            println!("error {:?}", e);
        }
    }

    Ok(())
}

fn apply_profile(s :&dyn SubShell, profile_name :String) -> Result<(), Box<dyn Error>> {
    let home :String ;
    match s.get_env(EnvQuery::Single(String::from("HOME")))? {
        EnvResult::Single(h) => {
            home = h;
        }
        EnvResult::Multi(_m) => panic!("invalid return value"),
    }

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