use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let s1 = &input;
    let s2 = &input;
    println!("{} {}", s1, s2);  

    som_fn(&mut input);

    let mut weight = calculate_weight_on_mars(77.6);
    weight *= 1000.0;
    
    println!("Weight on Mars: {}g", weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight/ 9.81) * 3.711
}

fn som_fn(s: &mut String){
    s.push_str("aa");
}
