use std::io;
use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
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
            // make Path
            let path = Path::new(&args[1]);
            // val for display file path
            let display = path.display();
            // open file at read-only
            let mut file = match File::open(&path) {
                Err(_) => panic!("couldn't open {}", display),

                Ok(file) => file, 
            };
            // string for read cmd
            let mut cmd = String::new();
            match file.read_to_string(&mut cmd){
                Err(_) => panic!("couldn't read {}", display),

                Ok(_) => println!("{}: {}", display, cmd), 
            }
        }

}

}

//fn interpreter(cmd: &str) -> String {  
//}
