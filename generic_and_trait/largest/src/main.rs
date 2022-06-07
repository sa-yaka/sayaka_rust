fn main() {
    let number_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'e'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
