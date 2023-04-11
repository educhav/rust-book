fn main() {
    // The formula for converting Fahrenheit to Celsius is C = 5/9(F-32)
    // 9C = 5F - 160
    // 9C + 160 = 5F
    // (9/5)C + 32

    println!("70f is: {}c", fahrenheit_to_celsius(70.0));
    println!("21c is: {}f", celsius_to_fahrenheit(21.111111));
    println!("32f is: {}c", fahrenheit_to_celsius(32.0));

}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    5.0/9.0 * (fahrenheit-32.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    ((9.0/5.0) * celsius) + 32.0
}
