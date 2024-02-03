fn main() {
    let mut strArray: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut strArray);
    println!("{:?}", strArray);
}

pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

pub fn reverse_string1(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..len / 2 {
        s.swap(i, ((len - 1) - i))
    }
}
pub fn reverse_string2(s: &mut Vec<char>) {
    let end = s.len() - 1;
    for i in 0..s.len() / 2 {
        let tmp = s[i];
        s[i] = s[end - i];
        s[end - i] = tmp;
    }
}
