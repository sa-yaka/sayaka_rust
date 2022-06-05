#![allow(unused)]

fn main() {
    let mut staff: [Employee; 3] = [
        Employee {
            name: String::from("Hack"),
            salary: 40000.0,
            age: 22,
        },
        Employee {
            name: String::from("Harry"),
            salary: 50000.0,
            age: 28,
        },
        Employee {
            name: String::from("Tony"),
            salary: 75000.0,
            age: 30,
        },
    ];

    for i in &mut staff {
        i.raise_salaru(10.0);
    }

    for i in staff {
        println!(
            "name = {}, salary = {}, age = {}",
            i.get_name(),
            i.get_salary(),
            i.get_age()
        );
    }
}

struct Employee {
    name: String,
    salary: f32,
    age: u8,
}

impl Employee {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_salary(&self) -> f32 {
        self.salary
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn raise_salaru(&mut self, by_percentt: f32) {
        self.salary += self.salary * by_percentt / 100.0
    }
}
