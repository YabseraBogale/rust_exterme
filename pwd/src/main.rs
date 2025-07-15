use std::env;
fn main() {
    let pwd = env::current_dir();
    match pwd {
        Ok(path) => {
            if let Some(p) = path.to_str() {
                println!("{}", p);
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
