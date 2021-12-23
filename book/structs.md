# Data types - Structs

Rust is not an object-oriented programming language. However, if you have a background in this king of languages, you will quickly find concepts that are similar to those of Object Oriented Programming (OOP), in particular with structs and methods that will be presented in this chapter.

## Iteration on employee email address generation

In the previous chapter, we implemented a function to generate the email address af any employee based on their first name and last name. The aim is to refactor what we did to improve readability.

It would be great to have a data structure that encapsulates every piece of information we know about the employee. For example, we could have the first name, the last name, the hiring date, the manager, the position and so on. It will quickly become cumbersome if we define a string for each attribute. Fortunately, Rust has a native data type for this use case: the `struct` data type.

Structs are a bit like classes in other programming languages, except there is no inheritance. Let's see how to manipulate them with an example.

## Refactor the `generate_email_address` function to take a `struct` as a parameter

Create a `struct` named `Employee` with the following fields:

* `first_name`
* `last_name`

Refactor the code (starting with the tests of course :wink:) so that the function takes this new data type as a parameter.

Your code should look like this:

```rust
const DOMAIN_NAME: &str = "mycompany.com";

struct Employee {
    first_name: String,
    last_name: String,
}

fn generate_email_address(employee: Employee) -> String {
    let first_name_first_letter = employee.first_name.to_lowercase().chars().next().unwrap();
    let last_name_with_conventions = employee.last_name.to_lowercase().replace(" ", "-");
    return format!(
        "{}.{}@{}",
        first_name_first_letter, last_name_with_conventions, DOMAIN_NAME
    );
}

#[test]
fn test_simple_case_english() {
    let fake_employee = Employee {
        first_name: String::from("roger"),
        last_name: String::from("federer"),
    };
    assert_eq!(
        generate_email_address(fake_employee),
        "r.federer@mycompany.com".to_string()
    )
}

#[test]
fn test_last_name_contains_spaces() {
    let fake_employee = Employee {
        first_name: String::from("roberto"),
        last_name: String::from("bautista agut"),
    };
    assert_eq!(
        generate_email_address(fake_employee),
        "r.bautista-agut@mycompany.com".to_string()
    )
}

#[test]
fn test_name_contains_uppercase_letters() {
    let fake_employee = Employee {
        first_name: String::from("Gael"),
        last_name: String::from("Monfils"),
    };
    assert_eq!(
        generate_email_address(fake_employee),
        "g.monfils@mycompany.com".to_string()
    )
}
```

New concepts have been introduced, so let's recap:

* you can define structs using the keyword `struct`.
* fields from a struct have types. In this case, they are the same but they can also be different.
* you create an instance by specifying values for the fields
* you access values of fields from an instance using the `.` notation

Tests pass! Yet I don't know about you but I have the feeling there is room for improvement.

## Introducing parameterized tests

Did you notice that tests are essentially the same? There's only a slight difference: the input and the expected output. But, we execute the duplicate the same code three times. When you see this pattern, the solution is often **paramaterized tests**. Parameterized tests allow you to execute the same test against a different set of inputs. Unfortunately, it is not included in the standard library but we can leverage some external pacakges.

Until then, you have used Cargo to run your code and your tests but in this chapter, you will also use it to manage your dependencies. To do so, you will write your dependencies in the `Cargo.toml` file that is automatically generated when you run `cargo new`.

To avoid reinventing the wheel, we will leverage a package called [parameterized](https://crates.io/crates/parameterized). There were other options but we will keep it simple with a package that does exactly what we need. To use it in your code add the following lines to your `Cargo.toml` file.

```toml
[dev-dependencies]
parameterized = "0.3.1"
```

{% hint style="info" %}
It is very easy in Rust to separate dependencies that are for the build phase and those that are necessary for the application to run. Add your app direct dependencies under `[dependencies]` and the libraries you need for development under `[dev-dependencies]`.
{% endhint %}

With parameterized tests, your code should now look like this:

```rust
#[cfg(test)]
extern crate parameterized;

const DOMAIN_NAME: &str = "mycompany.com";

struct Employee {
    first_name: String,
    last_name: String,
}

fn generate_email_address(employee: Employee) -> String {
    let first_name_first_letter = employee.first_name.to_lowercase().chars().next().unwrap();
    let last_name_with_conventions = employee.last_name.to_lowercase().replace(" ", "-");
    return format!(
        "{}.{}@{}",
        first_name_first_letter, last_name_with_conventions, DOMAIN_NAME
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(fake_employee = {
        Employee {
            first_name: String::from("roger"),
            last_name: String::from("federer"),
        },
        Employee {
            first_name: String::from("roberto"),
            last_name: String::from("bautista agut"),
        },
        Employee {
            first_name: String::from("Gael"),
            last_name: String::from("Monfils"),
        }
    }, expected = {
        String::from("r.federer@mycompany.com"),
        String::from("r.bautista-agut@mycompany.com"),
        String::from("g.monfils@mycompany.com")
    })]
    fn test_email_generation(fake_employee: Employee, expected: String) {
        assert_eq!(generate_email_address(fake_employee), expected)
    }
}
```

For now, you can forget everything around the test suite. It requires some knowledge about modules that will be covered later.

Thanks to parameterized tests, we can add a new test case very easily. Now we have refactored the tests, what about the business logic?

Passing the `Employee`'s struct to the function does not feel natural. It would be better to apply this function directly to the struct. For this, we use **methods**.

## Define a method instead of a function for email address generation

The syntax for methods is very close to the one for functions. THe only differences are:

* you need to encapsulate your function in a `impl <StructName> {}` block
* the first parameter is always `self`
* when you call them, you use the `.` notation on the instance

In your tests, there's only one line to change:

```rust
assert_eq!(fake_employee.generate_email_address(), expected)
```

And your method should look like this:

```rust
impl Employee {
    fn generate_email_address(&self) -> String {
        let first_name_first_letter = self.first_name.to_lowercase().chars().next().unwrap();
        let last_name_with_conventions = self.last_name.to_lowercase().replace(" ", "-");
        return format!(
            "{}.{}@{}",
            first_name_first_letter, last_name_with_conventions, DOMAIN_NAME
        );
    }
}
```

{% hint style="info" %}
In Rust and on the contrary to other languages like Python and Java, data definition is separated from methods in the code. Indeed, you use the `struct` keyword to define your fields and then you add methods with the `impl` keyword.
{% endhint %}

Verify your tests pass before moving on.

The code is clean but not idiomatic. In Rust, to create a new instance of a struct, it is common to see a call to a special method named `new`. This method acts similarly to constructors in other languages. It is a convention.

After this change, the final version of your code (yes I promise it is the last change) should look like the following snippet:

```rust
impl Employee {
    fn new(first_name: String, last_name: String) -> Employee{
        Employee{
            first_name: first_name,
            last_name: last_name
        }
    }

    fn generate_email_address(&self) -> String {
        ...
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(fake_employee = {
        Employee::new(String::from("roger"), String::from("federer")),
        Employee::new(String::from("roberto"), String::from("bautista agut")),
        Employee::new(String::from("Gael"), String::from("Monfils"))
    }, expected = {
        String::from("r.federer@mycompany.com"),
        String::from("r.bautista-agut@mycompany.com"),
        String::from("g.monfils@mycompany.com")
    })]
    fn test_email_generation(fake_employee: Employee, expected: String) {
        assert_eq!(fake_employee.generate_email_address(), expected)
    }
}
```

Much more concise right?

## Key takeaways

Although Rust does not have a `class` keyword, a lot of concepts from OOP can be applied through structs and methods. In Rust programs, structs are everywhere because they can group values of different types together. Fields are explicitely named which makes the code more readable. Sometimes, you want to apply a function on the instance of a struct and this is when methods come in.

Testing methods can generate a lot of boilerplate code and this is why it might be a good idea to declare multiple test cases for a single test function through parameterized tests. It is more digest to read.
