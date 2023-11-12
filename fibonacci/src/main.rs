use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter your number");

    io::stdin()
        .read_line(&mut n)
        .expect("Error reading the user input");

    let n: u8 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    for i in 0..n {
        let inner_n = i as f64;
        let square_of_5 = (5.0f64).sqrt();
        let phi = (1.0 + square_of_5) / 2.0;
        let fib: f64 = (phi.powf(inner_n) - (1.0 - phi).powf(inner_n)) / square_of_5;
        let fib = fib as i32;
        println!("Fib {i} is {fib}");
    }

    // let mut fn_1: u128 = 0;
    // let mut fn_2: u128 = 0;
    //
    // for i in 0..n {
    //
    //     if i == 1 {
    //         fn_1 = 1;
    //     } else {
    //         fn_1 = fn_1 + fn_2;
    //         fn_2 = fn_1 - fn_2;
    //     }
    //     let fib = fn_2 + fn_1;
    //     let inner = i+1;
    //     println!("Fib {inner} is {fib}");
    // }
}
