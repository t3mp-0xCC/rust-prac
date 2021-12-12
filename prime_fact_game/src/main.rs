use std::io;
use rand::Rng;

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

        match is_prime(input_prime.try_into().unwrap()) {
            false => println!("Not Prime !"),
            true => composite_num = composite_num / input_prime,
        }
    }

    println!("Factorizationed !");
}

fn is_prime(num: u32) -> bool {
    // Sieve of Eratosthenes
    // prime table
    let n: usize = num as usize;
    let mut sieve = vec![true; n+1];
    sieve[0] = false;
    sieve[1] = false;
    let sqrt_n = (f64::sqrt(n as f64) + 0.1).ceil() as usize;
    for i in 2..=sqrt_n {
        if !sieve[i] {
            continue;
        }
        for mult in ((i*i)..=n).step_by(i) {
            sieve[mult] = false;
        }
    }
   
    sieve[n]
}

fn composite_num_gen() -> u32 {
    let mut rand_num = 2;
    while is_prime(rand_num) {
        rand_num = rand::thread_rng().gen_range(2, 100);
    }

    rand_num
}