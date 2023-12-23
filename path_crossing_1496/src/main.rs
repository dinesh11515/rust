fn main() {
    println!("{}", is_path_crossing(String::from("NESWW")));
}
pub fn is_path_crossing(path: String) -> bool {
    let mut visited: Vec<(i8, i8)> = vec![(0, 0)];
    for ch in path.chars() {
        let mut last = visited.last().cloned().unwrap();
        match ch {
            'N' => last.1 += 1,
            'S' =>last.1 -= 1,
            'E' => last.0 += 1,
            'W' => last.0 -= 1,
            _ => {}
        }
        if (visited.contains(&last)) {
            return true;
        }
        visited.push(last);
    }
    false
}
