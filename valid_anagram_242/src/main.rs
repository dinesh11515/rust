use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}
pub fn is_anagram(s: String, t: String) -> bool {
    let mut sMap: HashMap<char, i32> = HashMap::new();
    let mut tMap: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        *sMap.entry(ch).or_insert(0) += 1;
    }
    for ch in t.chars() {
        *tMap.entry(ch).or_insert(0) += 1;
    }

    for (key, value) in sMap.iter() {
        match tMap.get(key) {
            Some(val) => {
                if val < value {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    pub fn test_is_anagram() {
        assert_eq!(
            is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
        assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
    }
}
