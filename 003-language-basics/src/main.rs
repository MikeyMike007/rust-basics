#![allow(unused_variables)]
#![allow(unused_assignments)]

// Constans
// ---------------------
const URL: &str = "google.com";
fn main() {
    println!("{}", URL);

    // Variables
    // -----------------
    let name = "Alex";
    let mut age = 32;

    let amount: i64 = 93876363636363636;

    println!("{}", age);
    age = 43;
    println!("{}", age);

    let color = "blue";
    println!("{}", color);
    let color = 86;
    println!("{}", color);

    let (a, b, c) = (43, 85, "red");

    // Strings
    // -----------------------
    let cat: &'static str = "Fluffy";
    let mut dog = String::from("Max"); // ERROR
    dog.push(' '); // Appends
    dog.push_str("the dog"); // Appends
    println!("{}", dog);
    let owner = format!("Hi iam the {} the owner of {}", "Mark", dog);
    println!("{}", dog.len());
    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);

    // Scalars
    // -------------------
    let pi: f32 = 4.0;
    let million = 1_000_000;
    println!("{}", million);
    let is_day: bool = true;
    let is_night: bool = false;
    println!("{}", is_day);
    let char1: char = 'A';
    let char1 = '\u{1F601}';
    println!("{}", char1);

    // Operators
    // --------------
    let a = 4 + 8;
    let b = 10 / 3;
    let c = 10 % 3;
    println!("a={}, b={}, c={}", a, b, c);
    println!("{}", a >= b);
    println!("{}", a >= b && b <= c);

    fn say_hello() {
        println!("Hello!");
    }

    fn greeting(name: &mut &str) -> String {
        let greeting = format!("Hello {}", name);
        greeting // return
    }

    for i in 1..6 {
        say_hello()
    }

    let mut my_name: &str = "John";
    let greet = greeting(&mut my_name);
    println!("{}", greet);
}
