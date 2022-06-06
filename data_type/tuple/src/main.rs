fn main() {
    let point = (2, 1, 6);
    let (x, y, z) = point;
    println!("{:?}", point);
    println!("{},{},{}", point.0, point.1, point.2);
    println!("{},{},{}", x, y, z);

    let hello = String::from("Hello World");
    let (s, len) = string_length(hello);
    println!("The length of '{}' is {}.", s, len);
}

fn string_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
