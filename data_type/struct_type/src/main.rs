#[derive(Debug)]

struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let mut sayaka = User {
        username: String::from("sayaka"),
        email: String::from("sayaka@example.com"),
        active: true,
    };
    println!("username: {}", sayaka.username);
    println!("email: {}", sayaka.email);
    println!("active: {}", sayaka.active);

    sayaka.email = String::from("sayaka@example.jp"); //要修改结构体中的事例需要将结构体申明为可变的。
    println!("now email: {}\n", sayaka.email);

    let admin = sign_up(String::from("admin"), String::from("admin@example.com"));
    println!("username: {}", admin.username);
    println!("email: {}", admin.email);
    println!("active: {}\n", admin.active);

    let syk = User {
        email: String::from("syk@example.jp"), // 只在 email 上与 sayaka 不同，只修改 email。
        ..sayaka
    };
    println!("username: {}", syk.username);
    println!("email: {}", syk.email);
    println!("active: {}\n", syk.active);
    // println!("username: {}", sayaka.username); error
    // String 类型没有实现 Copy 特征，所以 sayaka.username 的所有权被转移给了 sky。

    println!("{:?}", syk);
}

fn sign_up(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
    }
}
