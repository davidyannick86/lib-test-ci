#[cfg(test)]
mod tests {
    use crate::{are_both_true, longest};

    #[test]
    fn test_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_not_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        assert_ne!(result, "xyz");
    }

    #[test]
    fn test_are_both_true() {
        let expected = &4;

        let add = |x: &i32| *x + *x == *expected;
        let square = |x: &i32| *x * *x == *expected;

        let result = are_both_true(add, square, &2);
        assert!(result);
    }
}
