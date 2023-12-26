use std::result;

fn main() {
    println!("Hello, world!");
}

pub fn is_palindrome(x: i32) -> bool {
    let str = x.to_string();
    let mut start = 0;
    let mut end = str.len() - 1;

    while (start < end) {
        if (str.chars().nth(start).unwrap() == str.chars().nth(end).unwrap()) {
            start += 1;
            end -= 1;
        } else {
            return false;
        }
    }

    true
}

pub fn is_palindrome_nums(x: i32) -> bool {
    let mut rev = x;
    let mut result = 0;

    while rev > 0 {
        result = result * 10 + rev % 10;
        rev /= 10;
    }

    x == result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        // Standard case.
        assert_eq!(is_palindrome_nums(121), true);
        assert_eq!(is_palindrome_nums(12156), false);
        assert_eq!(is_palindrome_nums(3), true);
    }
}
