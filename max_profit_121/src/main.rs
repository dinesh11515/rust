fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));
}
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut min = prices[0];
    for i in prices {
        ans = ans.max(i - min);

        min = min.min(i);
    }
    ans
}
