# Data types - Strings

In the previous section, we had a look at a bunch of primitive types in Rust. We continue our series of tutorials on data types with strings. In most languages, strings are easy to manipulate. But the complexity is actually hidden behind high-level abstractions. In Rust, you might find string manipulation suprisingly difficult because the creators of the language decided to expose that complexity. Of course, it comes with benefits, and the main benefit is **control**.

To illustrate this chapter on strings, we will take the example of a program whose aim is to generate e-mail addresses from a first name and a last name.

## A minimalist version

Let's start with simple specs for now. The format of the e-mail address should be `<first name>.<last name>@mycompany.com`.

Your function and your first test should look like this:

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return "fails".to_string();
}

#[test]
fn test_simple_case(){
    assert_eq!(generate_email_address("roger", "federer"), "roger.federer@mycompany.com".to_string())
}
```

This block of code requires an explanation.

First, you may have noticed that I used two different types:

* the string slice `&str` type
* the string type

What are the differences between the two and why did we use string slices for the parameters and `String` as the return type?

## Two types of strings

Here is a short summary of the differences between the two types:

|   | `&str` | `String` |
| - | ------ | ----------- |
| What is it exactly? | A reference to some UTF-8 encoded string data stored elsewhere | A mutable UTF-8 enocded string type |
| What is the best use case? | ideal for text data that **does not** change | ideal for text data that changes |
| Is is natively supported? | Yes as part of the **core language** | **Yes** as part of the **standard library** |

There are also differences that are linked to how Rust manages memory but let's leave that aside for now. We will come back to that later.

Thanks to this table, you may have understood why we chose string slices for the function's parameters: the first name and last name won't change at runtime. On the contrary, the retrun value will be the result of a manipulation of the two parameters, so it is a good idea to use a mutable type.

## Implement the function body to complete the first version

Now we have a better understanding of the differences between `&str` and `String`, try to make the tests pass.

Your code may look something like this.

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    return String::from(first_name) + "." + &last_name + "@mycompany.com";
}

#[test]
fn test_simple_case(){
    assert_eq!(generate_email_address("roger", "federer"), "roger.federer@mycompany.com".to_string())
}
```

In terms of behaviour, `String::from()` and `to_string()` are equivalent, so you can change them interchangeably. Note that we had to convert the `first_name` variable to `String` whereas we did not for the `last_name` variable. The reason for that is that the `+` operator uses the `add` method behind the hood. The first parameter is `self` and the second is `s: &str`. So, you need to pass a `String` value as a first argument.

## Refactor the first version

We can do better.

* Harcoded values like `"mycompany.com"` are almost always a bad idea. It would be better to put this value aside from the function logic stored in a constant variable for example.
* The syntax with the `+` operator is not really readable and a bit heavy. Luckily for you, there is a macro that can help: `format!()`.

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

This looks better. Now we have made this simple version work. Let's add more requirements.

## More requirements

The e-mail address should now be composed of the first name's first character followed by a point and the last name in full. The following edge cases should also be covered:

* If the last name contains white spaces, they should be replaced with a dash character: "-"
* If the first name or the last name contains uppercase letters, they should be replaced with lowercase letters

As always, start with test implementation to cover these new requirements.

```rust
#[test]
fn test_simple_case_english(){
    assert_eq!(generate_email_address("roger", "federer"), "r.federer@mycompany.com".to_string())
}

#[test]
fn test_last_name_contains_spaces(){
    assert_eq!(generate_email_address("roberto", "bautista agut"), "r.bautista-agut@mycompany.com".to_string())
}

#[test]
fn test_name_contains_uppercase_letters(){
    assert_eq!(generate_email_address("Gael", "Monfils"), "g.monfils@mycompany.com".to_string())
}
```

Make the tests fail.

To make them pass, you need to tackle the following challenges:

* extract the first character of the first name
* replace spaces with dash characters
* convert uppercase letters to lowercase letters

The first challenge listed above is suprisingly the hardest. If you come from other languages, your reflex might be to try using indexing syntax. Try by yourself the following block of code:

```rust
let my_str = String::from("Rafael");
println!("first character: {}", my_str[0]);
```

If you configured your IDE to detect errors while typing, you may see an error message appear. If you didn't, try running `cargo run`. The compiler is not happy and tells you that ``Index<{integer}>` is not implemented for `String``. Why??? In fact, it has to do with the difference between the perception we have of a string as humans and the way your computer sees it.

Rust treats Strings as collections. And the problem is that a character does not always have a lentgth of 1. For example, try to guess the result of this expression: `"Дании́л".len()`. 6? You lost! The answer is 14 bytes! This is because each Unicode scalar value does not have the same length in bytes. Therefore, you understand why we can't use indexing. `"Дании́л"[0]` would not be a valid character.

But how can we extract the first character in all cases then?

You have two options. For both of them, you will use an iterator on all characters. Here are the two options:

* `text.chars().next().unwrap();`
* `text.chars().nth(0).unwrap();`

`.chars()` returns an iterator on characters and then you select the first one only with either `.next()` or `.nth(0)`. `.unwrap()` is here to catch errors, but we will come to error handling in a future chapter.

Ok, we have a solution for the first challenge, now on to the last two!

A research on the Web will help you find what you're looking for. The `to_lowercase()` and `replace()` functions will do the trick.

In the end, your solution should roughly look like this:

```rust
fn generate_email_address(first_name: &str, last_name: &str) -> String {
    let first_name_first_letter = first_name.to_lowercase().chars().next().unwrap();
    let last_name_with_conventions = last_name.to_lowercase().replace(" ", "-");
    return format!("{}.{}@{}", first_name_first_letter, last_name_with_conventions, DOMAIN_NAME);
}
```

Not so much to refactor, you already put a lot of efforts into this!

## Conclusion

If you don't have prior experience with low-level languages (C/C++ in particular), this chapter may have been frustrating. Strings in Rust need you to embrace the reasoning behind but once you understood the differences between `&str` and `String` and you know how to perform basic operations like concatenating or replacing characters, you are half way done!

## Go further

[Why Rust strings seem hard article](https://www.brandons.me/blog/why-rust-strings-seem-hard), Brandon Smith

[String type official documentation](https://doc.rust-lang.org/std/string/struct.String.html)

[Chapter on Strings in the official Rust book](https://doc.rust-lang.org/book/ch08-02-strings.html)
