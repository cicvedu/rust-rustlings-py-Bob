// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper {
    value: String,
}

impl Wrapper {
    pub fn new(value: String) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new("42".to_string()).value, "42");
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new(String::from("Foo")).value, "Foo");
    }
}