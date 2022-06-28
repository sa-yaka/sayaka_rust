fn main() {
    assert_eq!(sum(1, 2), 3);
}

// 通过两种方法使用特征约束来实现 `fn sum`
fn (&impl std::ops::Add) {
    x + y
}
