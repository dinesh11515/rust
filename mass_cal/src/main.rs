use std::io;
fn main() {
    println!("Enter the weight : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight:f32 = input.trim().parse().unwrap();
    println!("{}",calculate_mass_weight(weight));
}

fn calculate_mass_weight(weight: f32)-> f32{
    weight * 9.8 / 8.911 
}
