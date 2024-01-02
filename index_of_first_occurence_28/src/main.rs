fn main() {
    println!("{}", str_str(String::from("sabutsad"), String::from("sad")));
}
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() >= haystack.len() && haystack != needle {
        return -1;
    }

    for i in 0..haystack.len() - needle.len() + 1 {
        if (haystack[i..i + needle.len()] == needle) {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str(String::from("sadbutsad"), String::from("sad")), 0);
        assert_eq!(str_str(String::from("leetcode"), String::from("leeto")), -1);
        assert_eq!(str_str(String::from("sabutsad"), String::from("sad")), 5);
    }
}
