const FAHRENHEIT_FREEZING_POINT: i8 = 32;
const CELSIUS_FREEZING_POINT: i8 = 0;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FAHRENHEIT_FREEZING_POINT as f64
}

fn fahrenheit_to_celsius(c: f64) -> f64 {
    (c - FAHRENHEIT_FREEZING_POINT as f64) * 5.0 / 9.0
}

fn main() {

    let mut x = 40;

    for i in 0..=5 {
        println!("X is {} degrees fahrenheit", x);
        println!("Which is equivalent to {:.2} degrees Celsius", fahrenheit_to_celsius(x as f64));
        x+=1;
    }

    x = fahrenheit_to_celsius(x as f64) as i32;
    println!("X is {} degrees Celsius", x);
    println!("Which is equivalent to {:.2} degrees fahrenheit", celsius_to_fahrenheit(x as f64));
}
