use std::env;

fn main() {
   
    // make args to vector(string)
    let args:Vec<String> = env::args().collect();
    println!("args = {:?}" ,args);
    
}
