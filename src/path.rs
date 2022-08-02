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
use std::fmt::{Display, Formatter, Result as fmtResult};
use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Debug, Clone)]
struct NotInPathError;

impl Display for NotInPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "not found in PATH env variable")
    }
}

impl Error for NotInPathError {}

pub fn find_in_path(name: String) -> Result<String, Box<dyn Error>> {
    for p in crate::env_unicode::get_path()? {
        let binpath = format!("{}/{}", p, name);
        match metadata(Path::new(&binpath)) {
            Ok(_m) => {
                println!("success {}", binpath);
                return Ok(binpath);
            }
            Err(_e) => {
                continue;
            }
        }
    }

    Err(Box::new(NotInPathError))
}
