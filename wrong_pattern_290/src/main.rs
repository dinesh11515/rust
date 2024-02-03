use std::collections::HashMap;

fn main() {
    print!(
        "{}",
        word_pattern(String::from("jquery"), String::from("jquery"))
    );
}
pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let mut rmap: HashMap<&str, &str> = HashMap::new();

    let patterns: Vec<&str> = pattern.split("").collect();
    let objs: Vec<&str> = s.split(' ').collect();

    if (patterns.len() != objs.len() + 2) {
        return false;
    }
    for i in 0..objs.len() {
        let pat = *patterns.get(i + 1).unwrap();
        let obj = *objs.get(i).unwrap();
        if map.contains_key(pat) {
            print!("{} {}", pat, obj);
            if (map.get(pat).unwrap()) != &obj {
                return false;
            }
        } else if rmap.contains_key(obj) {
            if (rmap.get(obj).unwrap()) != &pat {
                return false;
            }
        } else {
            map.insert(pat, obj);
            rmap.insert(obj, pat);
        }
    }
    // print!("{:?}", patterns);
    true
}
