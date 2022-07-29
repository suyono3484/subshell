use std::env;

// fn coba() {
//     let i1 = 5;
//     let i2 = 7;
//     let i3 = 12;
//     let r1: i32;
//     let r2: i32;
//
//     r1 = i1 + i2;
//     r2 = i1 + i3;
//
//     println!("{} -> {}", r1, r2)
// }

fn main() {
    let p = subshell::prepare_zsh();
    match p {
        Ok(q) => {}
        Err(e) => {}
    }

    // println!("Hello, world!");
    // coba();
}
