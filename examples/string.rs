extern crate tein;
use tein::*;

fn main() {
    // read values from string
    let s: String = "123   34.5 true abc".to_owned();
    let mut teinner = Tein::new(s.as_bytes());

    let i: i32 = teinner.read().unwrap();
    let f: f32 = teinner.read().unwrap();
    let b: bool = teinner.read().unwrap();
    let s: String = teinner.read().unwrap();

    println!("i32:{}, bool:{}, f32:{}, String:{}", i, b, f, s);
}
