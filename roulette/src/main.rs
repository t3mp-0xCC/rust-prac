use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};
use rand::Rng;

fn main() {
    // show help
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: roulette <elements file>\nexp: roulette foods.txt");
        return;
    }

    // reading file and get elements as vec
    let path = Path::new(&args[1]);
    let roulette_elements = lines_from_file(path)
        .expect("file not found !");

    // show elements of file
    println!("Elements of Roulette");
    let mut i = 1;
    for element in roulette_elements.iter() {
        println!("{}: {:?}",i , element);
        i+=1;
    }

    // roulette time !
    let rand_num = rand::thread_rng().gen_range(1, roulette_elements.len());
    println!("***{:?}***", roulette_elements[rand_num]);

}

// reading file and split line into vec
fn lines_from_file(file: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}
