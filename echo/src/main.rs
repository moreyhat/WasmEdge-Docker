use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("At least one argument must be provided.")
    } else {
        let mut echo = "".to_string();
        for arg in &args[1..] {
            echo = echo + " " + arg.as_str();
        }

        println!("{}", echo);
    }
}
