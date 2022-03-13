fn main() {
    let number = 3;

    // 必须 是 bool 值。
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if
    let condition = true;

    // if 赋值中数据类型要匹配
    let number = if condition {
        5
    } else {
        7
    };
    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while 条件循环
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // 使用 for 循环遍历集合中的元素
    for element in a.iter(){
        println!("The value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
