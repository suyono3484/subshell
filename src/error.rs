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

use std::any::TypeId;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum SubShellError {
    SubShellDir(io::Error),
    CreateReserved(String),
    NoSuchProfileOrCommand(String),
    ProfileIsNotSet,
    InvalidSubShell,
}

impl Display for SubShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubShellError::SubShellDir(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        writeln!(f, ".subshell directory not found")
                    },
                    _other_error => {
                        writeln!(f, "cannot create/open directory .subshell")
                    },
                }
            },
            SubShellError::CreateReserved(name) => {
                writeln!(f, "cannot create profile name with reserved word {}", name)
            },
            SubShellError::NoSuchProfileOrCommand(command) => {
                writeln!(f, "{} profile is not found or it is an invalid command. Please refer to the help", command)
            },
            SubShellError::ProfileIsNotSet => {
                writeln!(f, "profile is not set")
            },
            SubShellError::InvalidSubShell => {
                writeln!(f, "invalid subshell")
            }
        }
    }
}

impl Error for SubShellError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SubShellError::SubShellDir(ref e) => Some(e),
            SubShellError::CreateReserved(_) => None,
            SubShellError::NoSuchProfileOrCommand(_) => None,
            SubShellError::ProfileIsNotSet => None,
            SubShellError::InvalidSubShell => None,
        }
    }
}

impl From<io::Error> for SubShellError {
    fn from(err :io::Error) -> SubShellError {
        SubShellError::SubShellDir(err)
    }
}