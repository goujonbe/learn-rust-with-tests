fn say_hello() -> String {
    return "Hello, world!".to_string()
}

fn main() {
    println!("{}", say_hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(), "Hello, world!");
    }
}
