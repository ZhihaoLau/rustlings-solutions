// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Scroll down for hints :)

mod for_test {
    pub fn is_it_cool() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use for_test::is_it_cool;

    #[test]
    fn you_can_assert_eq() {
        let cool = is_it_cool();
        assert_eq!(cool, true);
    }
}





























// Like the previous exercise, you don't need to write any code to get this test to compile and
// run. `assert_eq!` is a macro that takes two arguments and compares them. Try giving it two
// values that are equal! Try giving it two arguments that are different! Try giving it two values
// that are of different types! Try switching which argument comes first and which comes second!
