use std::env::{self, args};
fn main() {
    let args: Vec<String> = env::args().collect();
    for i in args {
        print!("{}", i);
    }
}
