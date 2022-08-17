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
use std::error::Error;
use subshell;
use subshell::SubShell;

fn main() -> Result<(), Box<dyn Error>> {
    let p = subshell::prepare_zsh();
    match p {
        Ok(q) => {
            subshell::profile::prepare(
                format!("{}/{}", q.get_home_env()?, String::from(".subshell")),
                Some(q.borrow()))?
                .set_profile_name(String::from("default"))
                .apply()?;
            q.exec()?;
        }
        Err(e) => {
            println!("error {:?}", e);
        }
    }

    Ok(())
}
