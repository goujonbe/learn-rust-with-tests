# Data types - Numbers and booleans

In this chapter, we will introduce integers, floating point numbers and boolean values. And what a better way than a math library to introduce number data types?

Let's create a new package with `Cargo`. If you want to create a lib instead of a binary program, you can use the `--lib` flag.

```shell
cargo new numbers_and_booleans --lib
```

Here are the specs of our math library:

* check if a number is even or odd
* calculate the hypotenuse of a right triangle using the [Pythagorean theorem](https://en.wikipedia.org/wiki/Pythagorean_theorem)

Let's start with some arithmetic. We want to implement a function that tells whether an integer is even (that is, if the result of a division by 2 is also an integer).

As usual, start by implementing a test.

```rust
fn is_even(number: u32) -> bool {
    return false;
}

#[test]
fn test_is_even() {
    assert!(is_even(4));
    assert_eq!(is_even(7), false);
}
```

The test fails as expected. Make it pass.

Your function should look similar to this:

```rust
fn is_even(number: u32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    return false;
}
```

A few remarks on this code snippet:

* The `%` operator calculates the arithmetic remainder. When equals to 0, the integer can be divided by 2.
* The syntax for conditional statements in Rust is very similar to what we find in other languages. You can use the `if`, `else if` and `else` keywords.
* You need to specify the type of your function parameters in the function's signature. Same applies for the return value.
* There is not a single integer type in Rust. Here we chose `u32`, because we only consider positive integers and we don't want to manipulate big numbers. But Rust includes a lot of different numeric types that are described in the following table.

| Types | Description |
| ----- | ----------- |
| `i8`, `i16`, `i32`, `i64` | Negative or positive numbers (signed numbers) that take up 8, 16, 32 or 64 bits of space |
| `u8`, `u16`, `u32`, `u64` | Positive numbers (unsigned numbers) that take up 8, 16, 32 or 64 bits of space |
| `f32`, `f64` | Floating point numbers, respectively 32 bits and 64 bits in size |
| `isize` | Signed numbers whose size depends on the machine the program is running on: 64 bits if you're on a 64-bit architecture, 32 bits otherwise  |
| `usize` | Unsigned numbers whose size depends on the machine the program is running on: 64 bits if you're on a 64-bit architecture, 32 bits otherwise  |

Navigating through all these types can be daunting. If you hesitate, go for the Rust default: `i32`.

Tests are green, how can we refactor the code above?

The body of the `is_even` function is quite verbose. We can shorten the code by returning the conditional expression directly. It would also be a good idea to separate our 2 assertions in 2 different tests to have more information in case of a failure thanks to the test's function name.

```rust
fn is_even(number: i64) -> bool {
    number % 2 == 0
}

#[test]
fn test_is_even() {
    assert!(is_even(4));
}

#[test]
fn test_is_odd() {
    assert_eq!(is_even(7), false);
}
```

{% hint style="info" %}
Note that the `return` keyword is not mandatory. As written in the book,

> The return value of the function is synonymous with the value of the final expression in the block of the body of a function.

{% endhint %}

To test your understanding, try to implement the `is_odd` function. Of course, start with a test!

Now we have the first feature of our math library, let's add the second one.

As a reminder, here is the formula we are going to use:

Given a right rectangle, if the lengths of both a and b are known, then c can be calculated as:

$$ c = \sqrt{a^{2} + b^{2}} $$

Apply this formula to write your first test case. Also add the function's signature and a dummy return value.

```rust
fn hypot(first_side: f64, second_side: f64) -> f64 {
    return 0.0
}

#[test]
fn test_hypot() {
    assert_eq!(hypot(2.5, 3.1), 3.9824615503479754)
}
```

Try to implement the function body on your own. Here is approximately what it should look like:

```rust
fn hypot(first_side: f64, second_side: f64) -> f64 {
    (f64::powi(first_side, 2) + f64::powi(second_side, 2)).sqrt()
}
```

The `::` syntax is used for both **associated functions** and **namespaces** created by **modules**. In this case, `powi()` is an associated function from the `f64` module. We will cover modules later in this book.

You may also have noticed that numbers in Rust can have methods, like the `sqrt()` method in our example. It is quite rare in programming languages.

The code is clean, not much refactoring to do. Of course, our test cases don't cover all scenarios. Testing is always a matter of trade-off, you can't cover all cases. The 100 % coverage should not always be your target. Be pragmatic and try to cover as much cases as possible without spending too much time on the implementation.

One default of our library is its lack of robustness. Indeed, we don't handle edge cases for now. For example, how would the `hypot()` function behave if we passed negative values? The solution would be to throw errors in these cases, but we have not seen how to deal with errors yet. We will refactor this library later in this book in the chapter on errors.

Coming up, strings!
