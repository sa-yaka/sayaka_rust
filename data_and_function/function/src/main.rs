fn main() {
    let x = 3;
    let y = 6;
    let z = "world";
    hello_world();
    hello_value(z);
    add(x, y);

    // 包含语句和表达式的函数体：
    let y = {
        let x = 3;
        x + 1 // 表达式的结尾没有分号，语句没有返回值。
    };
    println!("y = {}", y);

    let x = fave();
    println!("The value of x is: {}", x);

    let x = fave_add_one();
    println!("The value of x is: {}", x);
}

// 无参数函数：
fn hello_world() {
    println!("hello");
}

// 有参数函数：
fn hello_value(x: &str) {
    println!("hello {}", x);
}

fn add(x: i32, y: i32) {
    println!("The value is {}", x + y);
}

// 有返回值的函数：
fn fave() -> i32 {
    5
}

fn fave_add_one() -> i32 {
    fave() + 1
}