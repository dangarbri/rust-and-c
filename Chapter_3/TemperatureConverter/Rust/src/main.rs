// Convert temperatures from fahrenheit to celsius
fn fahrenheit_to_celsius(fahrenheit_temperature: i32) -> f32 {
    // Formula is (F - 32) / 1.8
    let temp_less_32 = fahrenheit_temperature - 32;
    return (temp_less_32 as f32) / 1.8;
}

fn main() {
    for fahrenheit in -40..41 {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F is {}°C", fahrenheit, celsius);
    }
}
