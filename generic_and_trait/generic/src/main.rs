fn main() {
    let p1 = Point {
        x: "hello",
        y: "world",
    };
    let p2 = Point {
        x: "你好",
        y: "rust",
    };

    let mix_p = p1.mixup(p2);
    println!("{}, {}!!", mix_p.x, mix_p.y);
}

#[derive(Debug)]
struct Point<T, V> {
    x: T,
    y: V,
}

impl<T, V> Point<T, V> {
    fn mixup<E, O>(self, other: Point<E, O>) -> Point<T, O> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
