pub mod staff_manager {
    pub struct Staff {
        name: String,
        salary: f64,
        age: u8,
    }
    mod set {}
    pub mod show {
        pub fn show_menu() {
            println!("1、添加职员");
            println!("2、显示职员");
            println!("3、删除职员");
            println!("4、修改职员");
            println!("5、清空职员");
        }
    }
}
impl Staff {
    fn hello() {
        println!("Hello");
    }
}
