// My name is Jesus Martinez
const FREEZING_POINT: f32 = 32.0;

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - FREEZING_POINT as f64) * (5.0 / 9.0) as f64
}
fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * (9.0 / 5.0) as f64) + FREEZING_POINT as f64
}

fn main() {
    let mut fahrenheit: f64 = 32.0;
    let mut num = 0; // iterator
    loop {
        //loop
        if num >= 6 {
            break; // break
        }

        let ftc: f64 = fahrenheit_to_celsius(fahrenheit); //fahrenheit to celsius
        println!("Celsius is {}*C\n", ftc); //print Celsius
        println!("Fahrenheit is {}*F\n", celsius_to_fahrenheit(ftc)); // celsius to fahrenheit
        num += 1; // increase iterator by 1
        fahrenheit += 1.0 as f64; // increase fahrenheit by 1
    }
}
