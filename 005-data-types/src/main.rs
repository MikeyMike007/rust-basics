#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use crate::Colors::Red;
use crate::ColorsGen::RedGen;
use crate::Person::Name;

fn main() {
    // Arrays
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);

    let numbers = [0; 15];
    println!("{:?}", doubles);

    const DEFAULT: i32 = 3;
    let numbers = [15, DEFAULT];

    println!("{:?}", numbers);
    // println!("{:?}", numbers[3]); // Panic

    // numbers[3] = 5; // Panic

    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // Vectors
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("{:?}", primes);

    primes.push(7);
    println!("{:?}", primes);

    primes.remove(2);
    println!("{:?}", primes);

    let mut numbers = vec![2; 10];
    println!("{:?}", numbers);

    const DEFAULT_NEW: bool = true;
    let values = vec![DEFAULT_NEW; 8];
    println!("{:?}", values);

    numbers[5] = 8;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number * number);
    }

    // Slices
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("numbers: {:?} ", numbers);
    println!("slice: {:?} ", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);

    // Tuples
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{:?}", person.0);
    println!("{:?}", person.1);
    println!("{:?}", person.2);
    person.0 = "Mike";
    println!("{:?}", person.0);

    // Destructuring
    let (name, age, employment) = person;
    println!("name: {}, age: {}, employed: {}", name, age, employment);

    // Structures
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35,
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_detail());

    // Enums
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);

    // Generics
    let p1: Point<i32, i32> = Point { x: 6, y: 8 };
    let p2: Point<f64, f64> = Point { x: 3.25, y: 8.63 };
    let p3: Point<i32, f64> = Point { x: 34, y: 8.5 };

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);

    let c1 = RedGen("#f00");
    let c2 = RedGen(255);
    let c3 = ColorsGen::<String>::BlueGen;

    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c3(String::from("#234")));
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

// Implementation
impl Employee {
    fn fn_details(&self) -> String {
        return format!(
            "name = {}, age = {}, company = {}",
            self.name, self.age, self.company
        );
    }

    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}

// Colors
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum ColorsGen<T> {
    RedGen(T),
    GreenGen(T),
    BlueGen(T),
}

#[derive(Debug)]
struct Point<T, V> {
    x: T,
    y: V,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
