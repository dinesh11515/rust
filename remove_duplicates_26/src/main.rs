fn main() {
    println!("{}", remove_duplicates(&mut vec![1, 1, 2]));
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut ans: i32 = 1;
    for i in 1..nums.len() {
        if (nums[i] != nums[i - 1]) {
            ans += 1;
            nums[ans as usize - 1] = nums[i]
        }
    }
    ans
}
