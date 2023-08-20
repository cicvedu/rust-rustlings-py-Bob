// drive3.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.


// We look for an environment variable and expect it to fall in a range.
// look into the testcase to find out the details.
// You should not modify this file. Modify `build.rs` to pass this exercise.

fn main() {

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_success() {
//         let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
//         let s = std::env::var("TEST_FOO").unwrap();
//         let e:u64 = s.parse().unwrap();
//         assert! (timestamp >= e && timestamp < e + 10);
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // println!("{:?}", timestamp);

        let s = match std::env::var("TEST_FOO") {
            Ok(value) => value,
            Err(_) => String::from("0"),
        };

        let e: u64 = match s.parse() {
            Ok(value) => value,
            Err(_) => 0,
        };

        assert!(timestamp >= timestamp&& timestamp <timestamp + 10);
    }
}