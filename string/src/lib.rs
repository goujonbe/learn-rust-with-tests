const DOMAIN_NAME: &str = "mycompany.com";

fn generate_email_address(first_name: &str, last_name: &str) -> String {
    let first_name_first_letter = first_name.to_lowercase().chars().next().unwrap();
    let last_name_with_conventions = last_name.to_lowercase().replace(" ", "-");
    return format!(
        "{}.{}@{}",
        first_name_first_letter, last_name_with_conventions, DOMAIN_NAME
    );
}

#[test]
fn test_simple_case_english() {
    assert_eq!(
        generate_email_address("roger", "federer"),
        "r.federer@mycompany.com".to_string()
    )
}

#[test]
fn test_last_name_contains_spaces() {
    assert_eq!(
        generate_email_address("roberto", "bautista agut"),
        "r.bautista-agut@mycompany.com".to_string()
    )
}

#[test]
fn test_name_contains_uppercase_letters() {
    assert_eq!(
        generate_email_address("Gael", "Monfils"),
        "g.monfils@mycompany.com".to_string()
    )
}
