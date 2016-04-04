extern crate tein;

use tein::*;

fn main() {
    // read n numbers from stdin to vector
    let mut teiner = new_with_stdin();
    let n: usize = teiner.read().unwrap();
    let nums: Vec<i32> = teiner.iter().take(n).collect();
    println!("{:?}", nums);
}
