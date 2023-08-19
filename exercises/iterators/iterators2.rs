// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
//func Complete the `capitalize_first` tion.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            if first.is_whitespace() {
                first.to_string() + c.as_str()
            } else {
                let capitalized = first.to_uppercase().to_string();
                capitalized + c.as_str()
            }
        }
        // Some(first) => {
        //     let capitalized = first.to_uppercase().to_string();
        //     capitalized + c.as_str()
        // }
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    capitalize_words_vector(words).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), vec!["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
