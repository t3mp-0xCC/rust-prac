use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    fmt::Display
};
use rand::Rng;

fn main() {
    /*TODO: if argc != 1, show help*/
    // args
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    // reading file and get elements as vec
    let roulette_elements = lines_from_file(path)
        .expect("file not found !");
    // show elements of file
    println!("Roulette of Elements");
    let mut i = 1;
    for element in roulette_elements.iter() {
        println!("{}: {:?}",i , element);
        i+=1;
    }
    // roulette time !
    let mut rand_num = rand::thread_rng().gen_range(1, roulette_elements.len());
    println!("***{:?}***", roulette_elements[rand_num]);

}

// reading file and split line into vec
fn lines_from_file(file: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}
