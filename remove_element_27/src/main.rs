fn main() {
    println!("{}", remove_element(&mut vec![1, 2, 2, 3], 1));
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ans: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[ans] = nums[i];
            ans += 1;
        }
    }
    ans as i32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        assert_eq!(remove_element(&mut vec![1, 2, 2, 3], 1), 3);
        assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(remove_element(&mut vec![3, 3], 3), 0);
    }
}
