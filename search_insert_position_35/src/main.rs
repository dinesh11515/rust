fn main() {
    println!("Hello, world!");
}
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if (target > nums[nums.len() - 1]) {
        return nums.len() as i32;
    }
    if (target < nums[0]) {
        return 0;
    }
    let mut start = 0;
    let mut end = nums.len();
    while (start < end) {
        let mid = start + (end - start) / 2;
        if (nums[mid] == target) {
            return mid as i32;
        } else if (nums[mid] < target) {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        // assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
