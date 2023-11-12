fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();
    println!("x = {}", x);

    let y = plus_one(x);
    println!("y = {}", y);
}

// fn main() {
//     let x = five();
//     println!("x = {}", x);
// }

// fn main() {
//     let y = {
//         let a = 1;
//         let z = 2 + a;
//         z + 3
//     };
//
//     println!("y = {}", y);
// }
//
