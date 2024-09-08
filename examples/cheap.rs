use std::collections::HashMap;

fn main() {
    let mut numbers = HashMap::new();
    for x in "abracadabra".chars() {
        let number = numbers.entry(x).or_insert(1);
        println!("{x}: {number}");
        *number += 1;
    }
}
