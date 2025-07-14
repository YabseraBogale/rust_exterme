use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    for i in args {
        print!("{} ", i);
    }
}
