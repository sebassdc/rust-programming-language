use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

// fn main() {
//     let x: (i32, f64, u8) = (200, 2.1, 2);
//
//     let two_hundred = x.0;
//
//     let two_point_one = x.1;
//
//     let two = x.2;
//
// }
