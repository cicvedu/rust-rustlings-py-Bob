// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.


fn modify_by_address(value: &mut u32) {
    *value = 0xAABBCCDD;
    // *value = 0xAA;
    // print!(O)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        let address = &mut t as *mut u32;
        modify_by_address(unsafe { &mut *address });
        assert!(t == 0xAABBCCDD);
    }
}