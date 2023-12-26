fn main() {
    println!(
        "{:?}",
        plus_one_other(vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 6
        ])
    );
}
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in digits.iter_mut().rev() {
        match *i == 9 {
            true => *i = 0,
            _ => {
                *i += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}

pub fn plus_one_other(mut digits: Vec<i32>) -> Vec<i32> {
    let len = digits.len();
    for i in (0..len).rev() {
        if (digits[i] < 9) {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    digits.insert(0, 1);
    digits
}
