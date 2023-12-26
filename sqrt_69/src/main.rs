fn main() {
    println!("{}", my_sqrt_binary(4));
}
// feasible for bigger numbers
pub fn my_sqrt_linear(x: i32) -> i32 {
    if (x == 0 || x == 1) {
        return x;
    }
    for i in 2..(x / 2) + 2 {
        if (i.pow(2) > x) {
            return i - 1;
        }
    }
    0
}

pub fn my_sqrt_binary(x: i32) -> i32 {
    if (x == 0 || x == 1) {
        return x;
    }
    let mut start = 1;
    let mut end = x / 2;

    while start < end {
        let mid: i32 = 1 + start + (end - start) / 2;
        let pro = x / mid;
        if (mid == pro) {
            return pro;
        } else if (pro > mid) {
            start = mid;
        } else {
            end = mid - 1;
        }
    }
    start
}
