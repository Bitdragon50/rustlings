// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


fn is_business_finished()-> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::is_business_finished;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(is_business_finished(), true);
    }
}
