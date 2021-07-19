use std::{env, process::exit};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn print_help() {
    println!("Usage: debugger <exec bin>");
}

fn main() {
     // arg check
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
            print_help();
            exit(-1);
    }
    // TODO: file check

    
    println!("temporary debugger\nwritten by Rust!");
}
