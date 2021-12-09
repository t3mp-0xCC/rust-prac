use std::io;
use std::env;

fn main() {
    /*TODO: read cmd from args if args exist*/
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments (read command from user input)
        1 => {
            let mut cmd = String::new();

            println!("Simple Brainfuck Interpreter");
            io::stdin().read_line(&mut cmd)
                .expect("Failed to read code !");
            //let output = interpreter(&cmd);

            //println!("{}", output);
        }
        // other (has more than one of arguments)
        _ => {
        }

}

fn interpreter(cmd: &str) -> String {
    
}
