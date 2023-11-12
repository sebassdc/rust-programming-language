use std::io;

fn main() {
    let mut user_farenheit = String::new();

    println!("Please enter your farenheit number");
    io::stdin()
        .read_line(&mut user_farenheit)
        .expect("Error encountered reading user input");

    let user_farenheit: f64 = match user_farenheit
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

    let celsius = (user_farenheit - 32.0) / 1.8;
    println!("{} farenheit is {} celsius", user_farenheit, celsius);
}
