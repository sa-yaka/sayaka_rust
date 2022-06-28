fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    {
        let _v2 = vec![1, 2, 3];
    } // _v2 在此被删除
      // println!("{:?}, _v2"); ERROR
    let _first = &v1[0]; // 不可变引用

    v1.push(6); // 可变引用
    println!("第二个元素是: {}", v1[2]);

    match v1.get(114) {
        Some(a) => println!("第114个元素是: {}", a),
        None => println!("没有第114个元素"),
    }

    // println!("第一个元素是: {:?}", _first); ERROR 不可变引用

    // 遍历 v1
    for (i, b) in v1.iter().enumerate() {
        println!("第 {} 个元素是：{}", i + 1, b);
    }

    // 改变元素的值
    for i in &mut v1 {
        print!("===");
        *i += 10;
    }
}
