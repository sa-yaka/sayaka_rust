fn main() {
    let mut first = 1;
    let mut second = 1;
    let mut third;
    let mut n = 10;
    loop {
        third = first + second;
        first = second;
        second = third;
        n -= 1;
        println!("Is now {}", third);
        if n - 2 == 0{
            break;
        }
    }
}
