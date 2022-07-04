fn main() {
    let _a = CustomSmartPointer {
        data: "a".to_string(),
    };
    let _b = CustomSmartPointer {
        data: "b".to_string(),
    };
    {
        let _c = CustomSmartPointer {
            data: "c".to_string(),
        };
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
