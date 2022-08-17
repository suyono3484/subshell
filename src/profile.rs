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

use std::error::Error;
use std::fs;
use std::path::Path;
use std::io;
use crate::error::SubShellError;
use crate::SubShell;

struct Profile<'a> {
    path : String,
    name : String,
    subshell : Option<&'a dyn SubShell>,
}

pub fn prepare<'a>(path : String, in_subshell : Option<&'a dyn SubShell>)
    -> Result<Box<dyn crate::Profile + 'a>, Box<dyn Error>> {
    let pcl = path.clone();
    let p = Path::new(&pcl);
    let retval = Profile{
        path,
        name: String::new(),
        subshell: {
            match in_subshell {
                Some(s) => Some(s),
                None => None,
            }
        },
    };
    match fs::metadata(p) {
        Ok(_) => Ok(Box::new(retval)),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    fs::create_dir(p)?;
                    Ok(Box::new(retval))
                },
                _ => Err(Box::new(SubShellError::SubShellDir(e))),
            }
        },
    }
}

impl Profile<'_> {
    fn subshell(&self) -> Result<&dyn SubShell, Box<dyn Error>> {
        // &self.subshell.un
        // match &self.subshell {
        //     Some(s) => {
        //         let c = s.;
        //
        //         Ok(c)
        //     },
        //     None => Err(Box::new(SubShellError::InvalidSubShell)),
        // }

        // if &self.subshell == None {
        //     Err(Box::new(SubShellError::InvalidSubShell))
        // } else {
        //     Ok(&self.subshell.unwrap_unchecked())
        // }
        match self.subshell {
            Some(s) => Ok(&*s),
            None => Err(Box::new(SubShellError::InvalidSubShell)),
        }
    }
}

impl crate::Profile for Profile<'_> {
    fn exist(&self) -> Result<&dyn crate::Profile, Box<dyn Error>> {
        match self.profile_path() {
            Ok(_) => Ok(self),
            Err(e) => Err(e),
        }
    }

    fn set_profile_name(&mut self, name: String) -> &mut dyn crate::Profile {
        self.name = name;
        self
    }

    fn profile_path(&self) -> Result<String, Box<dyn Error>> {
        if self.name.is_empty() {
            return Err(Box::new(SubShellError::ProfileIsNotSet));
        }

        let profile_dir = format!("{}/{}", &self.path, &self.name);
        match fs::metadata(Path::new(&profile_dir)) {
            Ok(_) => Ok(profile_dir),
            Err(e) => Err(Box::new(SubShellError::SubShellDir(e))),
        }
    }

    fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    fn apply(&self) -> Result<(), Box<dyn Error>> {
        let subshell = self.subshell()?;
        subshell.apply_environment(self)
    }
}