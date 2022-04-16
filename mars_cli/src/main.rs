use std::io;

fn main() {

    println!("Enter your weight in kg: ");
    let mut input = String::new();

    // may return an error
    io::stdin().read_line(&mut input).unwrap();

    // remove white space from input
    let weight: f32 = input.trim().parse().unwrap();

    // use input
    let mut mars_weight = calculate_weight_on_mars(weight);

    // convert to grams
    mars_weight = mars_weight * 1000.0;
    println!("This is your weight on Mars: {} g", mars_weight);

}

fn calculate_weight_on_mars(weight: f32) -> f32{

   (weight / 9.81) * 3.711
}

