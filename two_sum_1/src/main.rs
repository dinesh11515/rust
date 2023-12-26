use std::collections::HashMap;

fn main() {
    println!("{:?}", two_sum_hashmap(vec![3, 2, 4], 6));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
        let tar = target - nums[i];
        for j in i..len {
            if (tar == nums[j] && i != j) {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0]
}
pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, val) in nums.iter().enumerate() {
        let present_value = target - val;
        if map.contains_key(&present_value) {
            return vec![map[&present_value], i as i32];
        }
        map.insert(val, i as i32);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        // Standard case.
        assert_eq!(two_sum_hashmap(vec![1, 2, 3, 4, 5], 9), vec![3, 4]);
    }
}
