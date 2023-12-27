fn main() {
    println!("{}", roman_to_int(String::from("MCMXCIV")));
}
pub fn roman_to_int(s: String) -> i32 {
    let mut ans = 0;
    let mut last_char = 0;
    for (i, ch) in s.chars().rev().enumerate() {
        let val = match_roman(ch);
        if (val < last_char) {
            ans -= val;
        } else {
            ans += val;
        }

        last_char = val;
    }
    ans
}

fn match_roman(ch: char) -> i32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
