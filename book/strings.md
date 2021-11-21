first version

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return String::from(first_name) + "." + &last_name + "@mycompany.com";
}

#[test]
fn test_simple_case(){
    assert_eq!(generate_email_address("roger", "federer"), "roger.federer@mycompany.com".to_string())
}
```

refacto of first version

```rust
static DOMAIN_NAME: &str = "mycompany.com";

fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return format!("{}.{}@{}", first_name, last_name, DOMAIN_NAME);
}

#[test]
fn test_simple_case(){
    assert_eq!(generate_email_address("roger", "federer"), "roger.federer@mycompany.com".to_string())
}
```

Specifications:

In theory, the e-mail address should be composed of the first character of the first name followed by a point and the last name in full. The following edge cases should also be covered:

* If the last name contains white spaces, they should be replaced with a dash character: "-"
* If the first name or the last name contains uppercase letters, they should be replaced with lowercase letters

try to index the string
does not work
why?

this is because of how Rust stores strings in memory
https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation

slicing with bytes is possible, let's go

add test cases in russian and see that it fails
add a test case for Daniil Medvedev

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return format!("{}.{}@{}", &first_name[0..1], last_name, DOMAIN_NAME);
}

#[test]
fn test_simple_case_english(){
    assert_eq!(generate_email_address("roger", "federer"), "r.federer@mycompany.com".to_string())
}

#[test]
fn test_simple_case_russian(){
    assert_eq!(generate_email_address("Дании́л", "Медве́дев"), "Д.Медве́дев@mycompany.com".to_string())
}
```

problem because each character in the Cyrillic alphabet takes 2 bytes
Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value

so cannot index, but we can iterate on strings in other ways

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return format!("{}.{}@{}", first_name.chars().next().unwrap(), last_name, DOMAIN_NAME);
}
```

now it works!

explain `first_name.chars().next().unwrap()`

