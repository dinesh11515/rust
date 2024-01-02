use std::collections::HashMap;

fn main() {
    println!("{}", climb_stairs(44));
}
pub fn climb_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    pub fn climber(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if memo.contains_key(&n) {
            return *memo.get(&n).unwrap();
        }
        if (n == 0) {
            return 1;
        }
        let mut ans = 0;
        if (n - 1 >= 0) {
            ans = ans + climber(n - 1, memo);
        }
        if (n - 2 >= 0) {
            ans = ans + climber(n - 2, memo);
        }
        memo.insert(n, ans);
        ans
    };
    climber(n, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_insert() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        // assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
