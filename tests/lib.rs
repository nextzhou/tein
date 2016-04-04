extern crate tein;
use tein::{Tein, Input};
use std::io;

#[test]
fn read() {
    let s: String = "123   34.5 \n \t true".to_owned();
    // The std::io::Read trait has been implemented for &[u8] Type
    let mut teinner = Tein::new(s.as_bytes());

    let i: i32 = teinner.read().unwrap();
    let f: f32 = teinner.read().unwrap();
    let b: bool = teinner.read().unwrap();
    let e: Option<i32> = teinner.read();

    assert_eq!(i, 123);
    assert_eq!(f, 34.5);
    assert_eq!(b, true);
    assert_eq!(e, None);
}

#[test]
fn read_empty() {
    let mut teiner = Tein::new(io::empty());
    let s: Option<String> = teiner.read();
    assert_eq!(s, None);
    let i: Option<i32> = teiner.read();
    assert_eq!(i, None);
    let b: Option<bool> = teiner.read();
    assert_eq!(b, None);
}
