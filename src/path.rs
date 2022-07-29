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
