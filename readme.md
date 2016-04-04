# *Tein* - Tiny & Easy Inputer for [Rust](http://www.rust-lang.org)

A simple wrapping of inputting part of Rust standard library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.tein]
git = "https://github.com/nextzhou/tein.git"
```

### Stdin

```rust
// read n numbers from stdin to vector
extern crate tein;
use tein::*;

fn main() {
    // Same as: let mut teiner = Tein::new(std::io::stdin());
    let mut teiner = new_with_stdin();
    let n: usize = teiner.read().unwrap();
    let nums: Vec<i32> = teiner.iter().take(n).collect();
    println!("{:?}", nums);
}
```

### String

```rust
// read values from string
extern tein;
use tein::*;

fn main() {
    let s: String = "123   34.5 true abc".to_owned();
    let mut teinner = Tein::new(s.as_bytes());

    let i: i32 = teinner.read().unwrap();
    let f: f32 = teinner.read().unwrap();
    let b: bool = teinner.read().unwrap();
    let s: String = teinner.read().unwrap();

    println!("i32:{}, bool:{}, f32:{}, String:{}", i, b, f, s);

}
```

### Other type implemented std::io::Read trait.

Such as `std::fs::File`, `std::net::TcpStream`.
