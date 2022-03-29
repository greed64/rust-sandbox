fn main() {

    let weight = calculate_weight_on_mars(77.6);

    println!("Weight on Mars: {}kg", weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight/ 9.81) * 3.711
}

