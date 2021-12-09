use std::io;
use std::env;

fn main() {
    /*TODO: read cmd from args if args exist*/
    let mut cmd = String::new();

    println!("Simple Brainfuck Interpreter");
    io::stdin().read_line(&mut cmd)
        .expect("Failed to read code !");
    
    //let output = interpreter(&cmd);

    //println!("{}", output);

}

fn interpreter(cmd: &str) -> String {
    let mut stack = Vec<String>::new();

}
