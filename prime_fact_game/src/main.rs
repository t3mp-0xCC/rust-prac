use std::io;
use rand::Rng;

/*TODO: is_prime arg issue*/
/*TODO: Time Attack*/
/*TODO: Terminal UI*/

fn main() {
    let mut composite_num = composite_num_gen(); 
    let mut input_str = String::new();
    let mut input_prime;

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

fn is_prime(n: usize) -> bool {
    // Sieve of Eratosthenes
    // prime table
    let mut sieve = vec![true; n+1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..6{
        if sieve[i] {
            let mut j = i * i;
            while j < n+1 {
                sieve[j] = false;
                j += i;
            }
        }
    }
   
    sieve[n]
}

fn composite_num_gen() -> u32 {
    let mut rand_num: u32;
    while is_prime(rand_num) {
        rand_num = rand::thread_rng().gen_range(2, 100);
    }

    rand_num
}
