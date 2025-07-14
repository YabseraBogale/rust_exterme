use std::env::args;
fn main() {
    // format for head
    // head -n line_number file_name

    let args: Vec<String> = args().collect();

    if args[0] == "-n" {
        println!("ok");
    }
}
