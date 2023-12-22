fn main() {
    println!("{}", is_valid(String::from("()[}]")));
}
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        if (stack.is_empty()) {
            stack.push(char);
        } else {
            match stack.get(stack.len() - 1) {
                Some(ch) => match ch {
                    '(' => {
                        if (char == ')') {
                            stack.pop();
                        } else if (char == ']' || char == '}') {
                            return false;
                        } else {
                            stack.push(char);
                        }
                    }
                    '[' => {
                        if (char == ']') {
                            stack.pop();
                        } else if (char == ')' || char == '}') {
                            return false;
                        } else {
                            stack.push(char);
                        }
                    }
                    '{' => {
                        if (char == '}') {
                            stack.pop();
                        } else if (char == ')' || char == ']') {
                            return false;
                        } else {
                            stack.push(char);
                        }
                    }
                    _ => {
                        stack.push(char);
                    }
                },
                None => {}
            }
        }
    }
    stack.is_empty()
}
