#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
// use std::fmt::format;

// Need to add operator for operator overloading
use std::ops::Add;

// Structs
// ------------------

struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Traits
// -------------------

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello world");
    }
}

trait Bark {
    fn bark(&self) -> String;
}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

trait Summable<T> {
    fn sum(&self) -> T;
}

trait Duplicatable {
    fn dupl(&self) -> String;
}

// Implemenmtations
// ----------------------

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello World\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello(&self) {
        println!("system.out.println(\"Hello World\");");
    }
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for num in self {
            sum += num
        }
        sum
    }
}

// Operator overloading
// Add functionality for Point1 + Point2
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Duplicatable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicatable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

// Functions
// --------------------------
fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark())
}

/*
* The compiler needs to know the space required for a function return type
* A workaround is to return a box with a dyn trait
*
* fn get_animal(rand_number: f64) -> Animal {
*     if rand_number < 1.0 {
*         Dog {}
*     } else {
*         Cat {}
*     }
* }
*
* See solution below:
*/

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog { species: "Golden" })
    } else {
        Box::new(Cat { color: "Black" })
    }
}

// Static dispatch
fn duplicate<T: Duplicatable>(x: T) {
    println!("{}", x.dupl())
}

// Dynamic dispatch
fn duplicate_dynamic(x: &dyn Duplicatable) {
    println!("{}", x.dupl());
}

// Main
//----------------------------
fn main() {
    // 8. 41. Traits
    // ----------------------------

    // let r = RustDev { awesome: true }; // Instantiation
    let r: RustDev = RustDev::new(true);
    let j: JavaDev = JavaDev::new(true);
    println!("{}", r.language());
    r.say_hello();

    println!("{}", j.language());
    j.say_hello();

    println!("Are you awesome at rust: {}", r.awesome);
    println!("Are you awesome at java: {}", j.awesome);

    // 8.42. Generic Traits
    // -------------------------
    let dog = Dog {
        species: "retriever",
    };

    let _cat = Cat { color: "black" };
    bark_it(dog);
    // bark_it(cat); //ERROR - BARK is not implemented on Cat

    // 8.43. Returning traits
    println!("The animal says {}", get_animal(0.5).make_noise());
    println!("The animal says {}", get_animal(1.2).make_noise());

    // 8.44. Adding traints to existing structures
    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Sum of a: {}", a.sum());
    let b = vec![1.0, 2.0, 3.0];
    // println!("Sum float: {}", b.sum()) //ERROR DOES NOT WORK FOR FLOAT

    // 8.45. Operator overloading
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);
    println!("p1 + p2 = {:?}", p1 + p2);

    // 8.46. Static dispatch
    // Means that a generic trait will be converted to the required type at compile time
    // Monomorphization: Converting to one form
    //
    // CREATING MULTIPLE FUNCTIONS FOR A AND B AND REPLACE THEM AT COMPILE TIME
    let a = 42;
    let b = "Hi John".to_string();
    duplicate(a);
    duplicate(b);

    // 8.47. Dynamic dispatch
    // A generic trait will be converted to the required type at run time
    // CREATING MULTIPLE FUNCTIONS (2) FOR A AND B and CALLING THE RIGHT FUNCTION AT RUN TIME.
    duplicate_dynamic(&a);
    duplicate_dynamic(&b);
}
