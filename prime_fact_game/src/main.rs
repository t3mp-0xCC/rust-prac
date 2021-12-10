use std::io;
use rand::Rng;

/*TODO: is_prime*/
/*TODO: Time Attack*/
/*TODO: Terminal UI*/
fn main() {
    let mut composite_num = composite_num_gen(); 
    let mut input_str = String::new();
    let mut input_prime: u32;

    println!("Prime Factorization Game");

    while !is_prime(composite_num) {
        io::stdin().read_line(&mut input_str)
            .expect("Input Error !");
        input_prime = input_str.parse::<u32>().unwrap();

        match is_prime(input_prime) {
            false => println!("Not Prime !"),
            true => composite_num = composite_num / input_prime,
            _ => println!("Input Error !"),
        }
    }

    println!("Factorizationed !");
}

/*prime => 1, other => 0*/
fn is_prime(num: u32) -> bool { 
}

fn composite_num_gen() -> u32 {
    let mut rand_num: u32;
    while is_prime(rand_num) {
        rand_num = rand::thread_rng().gen_range(2, 100);
    }
}
