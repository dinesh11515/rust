fn main() {
    println!("{:?}", generate(6));
}
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }
    if num_rows == 2 {
        return vec![vec![1], vec![1, 1]];
    }
    let mut ans: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
    for i in 3..num_rows + 1 {
        let temp = cal(&ans[ans.len() - 1], i as usize);
        ans.push(temp)
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
