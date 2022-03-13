fn main() {
    // error：不能对不可变变量 x 二次赋值
    // let x = 5;
    // println!("x = {}", x);
    // x = 6;
    // println!("x = {}", x)


    // OK
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // OK
    let y = 5;

    println!("y = {}", y);

    let y = y + 1;

    println!("y = {}", y);

    // OK 实际上创建了一个新变量，可以改变值的类型。
    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces = {}", spaces);

    // array
    let a = [1, 3, 5, 6, 2];
    let first = a[1];
    println!("Array 1 = {}" , first);
}