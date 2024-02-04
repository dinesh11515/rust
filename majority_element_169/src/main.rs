use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for value in &nums {
        *map.entry(&value).or_insert(0) += 1;
        if map.get(&value).unwrap() > &(n / 2) {
            return *value;
        }
    }
    0
}

pub fn majority_element_moore(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut ele = &nums[0];
    for val in &nums {
        if count == 0 {
            ele = val;
        } else if ele == val {
            count += 1;
        } else {
            count -= 1;
        }
    }
    *ele
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_majority_element() {
        assert_eq!(majority_element_moore(vec![3, 2, 3]), 3);
    }
}
