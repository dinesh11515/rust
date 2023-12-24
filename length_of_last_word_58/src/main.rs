fn main() {
    println!(
        "{}",
        length_of_last_word(String::from("luffy is still joyboy"))
    );
}
pub fn length_of_last_word(s: String) -> i32 {
    let mut ans = 0;
    let mut started = false;
    let mut ended = false;

    for ch in s.chars().rev() {
        match ch {
            ' ' => {
                if (started) {
                    ended = true;
                }
            }
            _ => {
                if (ended && ans > 0) {
                    return ans;
                }
                ans += 1;
                started = true;
            }
        }
    }
    ans
}
