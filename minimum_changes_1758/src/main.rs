fn main() {
    println!("{}", min_operations(String::from("0100")));
}

pub fn min_operations(s: String) -> i32 {
    let mut ans1 = 0;
    let mut ans2: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            '0' => {
                if (i % 2 != 0) {
                    ans1 += 1;
                } else {
                    ans2 += 1;
                }
            }
            '1' => {
                if (i % 2 == 0) {
                    ans1 += 1;
                } else {
                    ans2 += 1;
                }
            }
            _ => {}
        }
    }

    if (ans1 > ans2) {
        ans2
    } else {
        ans1
    }
}
