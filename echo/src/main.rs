use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    println!();
    for i in args {
        print!("{}", i);
    }
}
