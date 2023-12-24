fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3]));
}
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut sum = String::new();
    for i in digits {
        sum.push((i as u32).to_str());
    }
    let mut ans: Vec<i32> = vec![];
    for ch in sum.to_string().chars() {
        ans.push(ch as i32);
    }
    ans
}
