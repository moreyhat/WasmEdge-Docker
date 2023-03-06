use std::env;

fn fibonacci(n: &u32) -> u32 {
    if n.le(&1) {
        return *n;
    }
    fibonacci(&(n - 1)) + fibonacci(&(n - 2))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len().eq(&2) {
        println!("One argument is requred.");
        return;
    }
    match args[1].parse::<u32>() {
        Ok(n) => println!(
            "The fibonacci consequence of index {} is {}",
            n,
            fibonacci(&n)
        ),
        Err(_) => println!("The argument needs to be number"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(&1), 1);
        assert_eq!(fibonacci(&1), 1);
        assert_eq!(fibonacci(&11), 89);
        assert_eq!(fibonacci(&20), 6765);
    }
}
