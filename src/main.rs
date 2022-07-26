use std::env;

fn main() {
    let path_key = "PATH";
    match env::var_os(path_key) {
        Some(ospath) => if let Ok(path) = ospath.into_string() {
            println!("got path {}", path);
            for p in path.split(":") {
                println!("path item: {}", p);
            }
        } else {
            println!("invalid unicode in PATH env variable");
        },
        None => println!("PATH is not set"),
    }

    println!("Hello, world!");
}
