use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
pub fn single_number_map(nums: Vec<i32>) -> i32 {
    let mut tracker: HashMap<i32, i32> = HashMap::new();
    for i in nums {
        let val = tracker.get(i).unwrap().insert(i, 0);
    }
    0
}

pub fn single_number_sort(nums: &mut Vec<i32>) -> i32 {
            nums.sort();
            for i in 1..nums.len() - 1 {
                if (nums[i] != nums[i - 1] || nums[i] != nums[i + 1]) {
                    return nums[i];
                }
            }
            if (nums[1] != nums[0]) {
                return nums[0];
            } else {
                return nums[nums.len() - 1];
            }
            0
}

pub fn single_number_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for i in nums {
        result = result ^ i;
    }

    result
}
