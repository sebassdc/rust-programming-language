fn main() {
    let mut s = String::from("hello");

    println!("s: {s}");

    change(&mut s);
    change(&mut s);

    println!("s: {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
