extern crate rand;
use rand::RNG;
fn main() {
    let random_number=rand::thread_rng().gen_range(1,20);
    println!("Random Number is {}",random_number);
}
