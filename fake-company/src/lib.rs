#[cfg(test)]
extern crate parameterized;

const DOMAIN_NAME: &str = "mycompany.com";

struct Employee {
    first_name: String,
    last_name: String,
}

impl Employee {
    fn new(first_name: String, last_name: String) -> Employee {
        Employee {
            first_name: first_name,
            last_name: last_name,
        }
    }

    fn generate_email_address(&self) -> String {
        let first_name_first_letter = self.first_name.to_lowercase().chars().next().unwrap();
        let last_name_with_conventions = self.last_name.to_lowercase().replace(" ", "-");
        return format!(
            "{}.{}@{}",
            first_name_first_letter, last_name_with_conventions, DOMAIN_NAME
        );
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
