use std::env::args;
use std::fs;
fn main() {
    // format for head
    // head file_name

    let args: Vec<String> = args().collect();
    let content = fs::read_to_string(args[1].as_str()).expect("File does not exist");
    let mut counter = 0;
    for i in content.split("\n") {
        counter += 1;
        println!("{}", i);
        if counter == 5 {
            break;
        }
    }
}
