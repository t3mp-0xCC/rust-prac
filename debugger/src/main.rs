use std::{env, process::exit};

fn init() {
    // arg check
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
            print_help();
            exit(-1);
    }
}

fn print_help() {
    println!("Usage: debugger <exec bin>");
}

fn main() {
    init();
    println!("temporary debugger\n written by Rust!");
}
