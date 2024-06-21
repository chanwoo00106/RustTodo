use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your weight (kg): ");
    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim().parse::<f32>().unwrap();

    println!("Weight on Mars: {}kg", calculate_weight_on_mars(weight));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
