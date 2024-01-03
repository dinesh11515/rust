fn main() {
    println!("{:?}", fizz_buzz(3));
}
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut ans: Vec<String> = vec![String::new(); n];
    for i in 1..n + 1 {
        if (i % 3 == 0 && i % 5 == 0) {
            ans[i - 1] = String::from("FizzBuzz");
        } else if i % 3 == 0 {
            ans[i - 1] = String::from("Fizz");
        } else if i % 5 == 0 {
            ans[i - 1] = String::from("Buzz");
        } else {
            ans[i - 1] = i.to_string();
        }
    }
    ans
}
