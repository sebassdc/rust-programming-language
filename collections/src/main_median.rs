use std::collections::HashMap;

fn main() {
    let mut integers = vec![10, 10, 10, 10, 50, 20, 30, 10, 40, 40, 40];
    println!("integers: {integers:?}");

    integers.sort();
    println!("integers: {integers:?}");

    let median = integers[integers.len() / 2];
    println!("median: {median}");

    let mut mapped_ints = HashMap::new();

    for number in integers {
        let key_val_ref = mapped_ints.entry(number).or_insert(0);
        *key_val_ref += 1;
    }

    let mode = mapped_ints.iter().max_by_key(|e|e.1).unwrap();
    println!("mode: {}", mode.0);
}
