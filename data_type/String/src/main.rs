fn main() {
    // _set_string();
    // _string_slice();
    _string_catenate();
}

// 字符串切片
fn _string_slice() {
    let hello = String::from("Hello Rust");
    let hello_slice = &hello[6..]; // 切片是对 String 类型的一种引用。
    println!("hello: {:?}", hello);
    println!("hello slice: {:?}", hello_slice);

    println!("first: {:?}", _first_word(&hello));
    // hello.clear(); clear() 方法需要一个可变引用，而 first() 方法需要一个不可变引用
}

fn _first_word(s: &String) -> &str {
    &s[..1]
}

// 操作字符串
fn _set_string() {
    let mut hello = String::from("Hello World");
    println!("string: {:?}", hello);

    hello.push(','); // 追加字符
    println!("push ',': {:?}", hello);

    hello.push_str(" Rust"); // 追加字符串
    println!("push_str \" Rust\": {:?}", hello);

    hello.insert(5, ',');
    println!("insert ',': {:?}", hello); // 插入字符

    hello.insert_str(13, " Hello"); // 插入字符串
    println!("insert_str \" Hello\": {:?}", hello);
    // 使用以上四种方法时，字符串变量必须是可变的。

    let hello_replace = hello.replace("Hello", "Rust"); // replace 替换，该方法返回一个新的字符串，适用于 String 和 &str 类型。
    println!("hello_replace: {:?}", hello_replace);

    let hello_replacen = hello.replacen("Hello", "Rust", 1); // replacen 前两个参数与 replace 一样，第三个参数表示要替换的个数。
    println!("hello_replacen: {:?}", hello_replacen);
    // 以上两种方法适用于 String 和 &str 类型，该方法返回一个字符串。

    hello.replace_range(..19, "Hello"); //replace_range 接收两个参数，的一个是替换的范围，第二个是替换的内容。
    println!("hello_replace_range: {:?}", hello); // 该方法直接操作字符串。

    hello.push('!');
    println!("hello: {:?}", hello);
    hello.pop();
    println!("hello.pop(): {:?}", hello); // pop(index) 直接操作字符串，删除最后一个字符。存在一个 Option 的返回值，，如果字符串为空则返回 None。

    hello.insert(6, 'w');
    println!("hello: {:?}", hello);
    hello.remove(6); // remove(index) 直接操作字符串，删除指定位置的字符。该方法 按照字节来处理字符串，需要指定合法的字符边界。
    println!("hello.remove(6): {:?}", hello);

    hello.truncate(5);
    println!("hello.truncate(5): {:?}", hello); //truncate(index) 删除字符串中从指定位置开始到结尾的全部字符。

    hello.clear(); // clear() 删除字符串中的所有字符。
    println!("hello.clear(): {:?}", hello);
    // 以上四种删除字符串的方法都是直接操作字符串。
}

fn _string_catenate() {
    let string_hello = String::from("Hello");
    let string_rust = String::from("Rust");

    let hello = string_hello + " ";
    println!("{:?}", hello);

    let result = hello + &string_rust; // 相当于调用了 std::string 标准库中的 add() 右边的参数必须是字符串切片类型, + += 都是返回一个新的字符串。
    println!("{:?}", result);
    // println!("{:?}", string_hello); string_hello 的所有权被移走了，因此报错。
}
