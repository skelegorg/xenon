use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    lib::run(args);
}

