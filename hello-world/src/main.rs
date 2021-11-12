fn say_hello() -> String {
    return "Hello, world!".to_string()
}

fn main() {
    println!("{}", say_hello());
}

#[test]
fn test_say_hello() {
    assert_eq!(say_hello(), "Hello, world!");
}
