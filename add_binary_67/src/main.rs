fn main() {
    println!("{}", add_binary(String::from("1001"), String::from("1001")));
}
pub fn add_binary(a: String, b: String) -> String {
    let mut ans = String::new();
    let mut len = 0;
    if a.len() < b.len() {
        len = a.len();
    } else {
        len = b.len();
    };
    let mut extra = 0;
    for i in 0..len {
        let ch1 = &a[a.len() - 1 - i..a.len() - i];
        let ch2 = &b[b.len() - 1 - i..b.len() - i];
        if ch1 == "0" && ch2 == "0" {
            if extra == 1 {
                ans.push('1');
                extra = 0;
            } else {
                ans.push('0');
            }
        } else if ch1 == "0" && ch2 == "1" {
            if extra == 1 {
                ans.push('0');
                extra = 1;
            } else {
                ans.push('1');
            }
        } else if ch1 == "1" && ch2 == "0" {
            if extra == 1 {
                ans.push('0');
                extra = 1;
            } else {
                ans.push('1');
            }
        } else if ch1 == "1" && ch2 == "1" {
            if extra == 0 {
                ans.push('0');
                extra = 1;
            } else {
                ans.push('1');
            }
        }
    }
    if len < a.len() {
        for i in len..a.len() {
            let ch1 = &a[a.len() - 1 - i..a.len() - i];
            if ch1 == "0" {
                if extra == 0 {
                    ans.push('0');
                } else {
                    ans.push('1');
                    extra = 0;
                }
            } else {
                if extra == 0 {
                    ans.push('1');
                } else {
                    ans.push('0');
                    extra = 1;
                }
            }
        }
    }
    if len < b.len() {
        for i in len..b.len() {
            let ch1 = &b[b.len() - 1 - i..b.len() - i];
            if ch1 == "0" {
                if extra == 0 {
                    ans.push('0');
                } else {
                    ans.push('1');
                    extra = 0;
                }
            } else {
                if extra == 0 {
                    ans.push('1');
                } else {
                    ans.push('0');
                    extra = 1;
                }
            }
        }
    }
    if extra == 1 {
        ans.push('1');
    }
    ans.chars().rev().collect::<String>()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    pub fn test_add_binary() {
        assert_eq!(
            add_binary(String::from("11"), String::from("1")),
            String::from("100")
        );
        assert_eq!(
            add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        );
        assert_eq!(
            add_binary(String::from("1001"), String::from("1001")),
            String::from("10010")
        )
    }
}
