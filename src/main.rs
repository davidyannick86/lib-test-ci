fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn are_both_true<T, U, V>(f1: T, f2: U, number: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(number) && f2(number)
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let expected = &4;

    let add = |x: &i32| *x + *x == *expected;
    let square = |x: &i32| *x * *x == *expected;

    let result = are_both_true(add, square, &2);
    println!("Are both true : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

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
