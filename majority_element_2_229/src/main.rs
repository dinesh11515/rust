use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32 / 3;
    let mut map: HashMap<&i32, i32> = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    for value in &nums {
        *map.entry(&value).or_insert(0) += 1;
        if !ans.contains(&value) && map.get(&value).unwrap() > &n {
            ans.push(*value);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_fn_majority_element() {
        assert_eq!(majority_element(vec![3, 3]), vec![3]);
    }
}
