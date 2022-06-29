use std::collections::HashMap;

fn main() {
    let s = "MCMXCIV".to_string();
    let n = roman_to_int(s);
    println!("n = {}", n);
}

fn roman_to_int(s: String) -> i32 {
    let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
        .iter()
        .zip([1, 5, 10, 50, 100, 500, 1000])
        .collect::<HashMap<_, _>>();
    s.chars().rev().fold(0, |acc, c| {
        if acc > 4 * map[&c] {
            acc - map[&c]
        } else {
            acc + map[&c]
        }
    })
}
