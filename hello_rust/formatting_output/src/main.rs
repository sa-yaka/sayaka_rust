fn main() {
    // `{}` 会被任意变量内容所替换。
    // 变量内容会被转为字符串。
    println!("{} days", 31);
    // 不加后缀的话，31 就自动成为 32(32-bit) 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 在 ':' 后面指定输出格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 指定宽度 将打印 0000001。
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);
    
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct '{}' won't print...", Structure(3));
    
    let pi = 3.14159;
    println!("PI is roughly {:.3}", pi);
}
