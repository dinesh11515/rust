fn main() {
    println!("{:?}", get_row(3));
}
pub fn get_row(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
        return vec![1];
    }
    if row_index == 1 {
        return vec![1, 1];
    }
    let mut ans = vec![1, 1];
    for i in 3..row_index + 2 {
        ans = cal(&ans, i as usize);
    }
    ans
}

pub fn cal(input: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![1; n];
    let mut start: usize = 1;
    let mut end = n - 2;
    while start <= end {
        let value = input[start] + input[start - 1];
        println!("{:?}", value);
        println!("{:?},{}", start, end);

        ans[start] = value;
        ans[end] = value;
        start += 1;
        end -= 1;
    }
    ans
}
