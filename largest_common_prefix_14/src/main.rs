fn main() {
    let mut strs: Vec<String> = Vec::new();
    strs.push(String::from("flower"));
    strs.push(String::from("flow"));
    strs.push(String::from("flight"));

    println!("{}",longest_common_prefix(strs));
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ans = String::new();
    let first_word = &strs[0];
    for i in 1..first_word.len()+1{
        let ch = &first_word[0..i];
        for text in strs.iter() {
            if(!text.starts_with(ch)){
                return ans;
            }
        }
        ans = ch.to_string();
    }

    return ans;
}