use std::collections::{HashMap, HashSet};
use std::iter::zip;
fn main() {
    println!("Hello, world!");
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    // let mut map: HashMap<char, char> = HashMap::new();
    // for (i, char) in s.chars().enumerate() {
    //     if map.contains_key(&char) {
    //         if map.get(&char).unwrap() != &t.chars().nth(i).unwrap() {
    //             return false;
    //         }
    //     } else {
    //         if map.con {
    //             return false;
    //         }
    //         map.insert(char, t.chars().nth(i).unwrap());
    //     }
    // }
    // true
    let mut map: HashMap<char, char> = HashMap::new();
    for (i, char) in s.chars().enumerate() {
        let val = map.entry(char).or_insert(t.chars().nth(i).unwrap());

        if val != &t.chars().nth(i).unwrap() {
            return false;
        }
    }
    map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
}

pub fn is_isomorphic_2(s: String, t: String) -> bool {
    let mut d: HashMap<char, char> = HashMap::new();
    let mut d1: HashMap<char, char> = HashMap::new();
    for (s1, t1) in zip(s.chars(), t.chars()) {
        if !d.contains_key(&s1) && !d1.contains_key(&t1) {
            d.insert(s1, t1);
            d1.insert(t1, s1);
        } else {
            if d.get(&s1) != Some(&t1) && d1.get(&t1) != Some(&s1) {
                return false;
            }
        }
    }
    return true;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            is_isomorphic(String::from("add"), String::from("egg")),
            true
        );
        assert_eq!(
            is_isomorphic(String::from("foo"), String::from("bar")),
            false
        );
        assert_eq!(
            is_isomorphic(String::from("badc"), String::from("baba")),
            false
        );
    }
}
