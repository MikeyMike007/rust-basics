use std::rc::Rc;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    /* 9.49. Ownership
     * -------------------------
     * Only one variable can own a piece of memory. When we create a variable, we are basically
     * creating a reference to a specific part of the memory that is uniqly assicoated with this
     * variable. This provides memory guarantee i.e. no other variable can change that part of
     * memory. When we trandfer ownershipm, that impåacts how we can use those two variables that
     * are involved in this transaction
     *
     * For primitive types (integers, booleans, floats):
     * Copy data is cheap since we know the amount of memory that is required for that transaction.
     * When we change ownership of data, we do that through copying a variable to another
     * primitive.
     *
     * For complex types (structures etc): Ownership is transferred. We do not know the size at
     * compile time so that means that we can not make guarantees for memory at compile time when we
     * transfer ownership. When we make an operation with a complex type, ownership is transferred.
     */

    // Primitive types
    // These two variables still retain ownership of this piece of data. Data has been copied to
    // the new variable.
    let i: i32 = 5;
    let j: i32 = i;
    println!("i: {}", i);
    println!("j: {}", j);

    // Complex type
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);
    let w: Vec<i32> = v; // NEW OWNERSHIP

    println!("w: {:?}", w);
    // print!("v: {:?}", v); // ERROR - OWNERSHIP HAS CHANGED

    // WHat happends in lambda expressions

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vecor used in foo");
        v
    };
    // Ownership doesnt change here, v1 still points at the same memory
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v1: Vec<i32> = foo(v1);
    println!("v1: {:?}", v1);

    /* 9.50. Borrowing
     * ------------------------------------------------------
     *  Ownership of a piece of memory is borrowed to another variable temporarily.
     *  SO,
     *  1) Only one variable can own a piece of memory
     *  2) Variables can borrow ownership to other peices of memory
     *
     *  Illustration:
     *  let a = 6;
     *  let b = &a; // BORROWING
     *  println!("b: {}", *b); // DEREFERENCE
     *
     *  PLEASE NOTE THAT THE BORROW MUST MATCH MUTABILITY
     */
    println!("---------------------------");
    let mut a: i32 = 6;
    println!("a = {}", a);
    let b: &mut i32 = &mut a;
    println!("*b = {}", *b); // Borrowing the variable b as mutable
    println!("a = {}", a); // Borrowing the variable b as imutable
                           // *b += 2; // Borrowing as a mutable again - THIS CAUSES PROBLEMS

    println!("---------------------------");

    // WHAT CAN WE DO? SOLUTION 1
    let mut a1: i32 = 6;
    println!("a1 = {}", a1);
    let b1: &mut i32 = &mut a1;
    println!("*b1 = {}", *b1);
    *b1 += 2;
    println!("a1 = {}", a1);

    println!("---------------------------");

    // WHAT CAN WE DO? SOLUTION 2
    let mut a2: i32 = 6;
    println!("a2 = {}", a2);
    {
        // Variable b2 will be destroyed when the scope finishes
        let b2: &mut i32 = &mut a2;
        println!("*b2 = {}", *b2);
        *b2 += 2;
    }

    println!("a2 = {}", a2);

    println!("---------------------------");

    // GOOD EXAMPLE OF MEMORY GUARANTEE IN RUST
    // WITH THIS CODE, WE CAN HAVE AN INFINITE AMOUNT OF ELEMENTS COMING INTO v
    // THIS WILL FILL UP ALL MEMORY
    // let mut v = vec![1, 2, 3, 4, 5];
    // for i in &v {
    //     println!("i = {}", i);
    //     v.push(6); //ERROR! WE CANNOT BORROW the variable v as mutable as it is already borrowed as immutable.
    // }

    /* 9.51. LIFETIME
     *
     * LIFETIME is an indication of how long an object will live
     *
     * In the example below, we have a dog object with a reference to a Person object. If the part
     * of memory where owner is pointing (Person with name "john") becomes invalid for some reason.
     * Then the owner will be pointing to invalid data. Rust doesnt accept that. How can we solve
     * this problem? WE CAN USE THE LIFETIME SPECIFIER!! 🎉
     *
     * THIS SPECIFIER WILL TELL THE SYSTEM THAT THE REFERENCE WILL LIFE AS LONG AS PERSON (p1) IS LIVING.
     *
     * This means Rust prevents parts of obhects outliving the object
     *
     * struct Object<'lifetime> {
     * field: &'lifetime str
     * }
     */
    println!("{}", get_str());

    let p1 = Person {
        name: String::from("John"),
    };

    let d1 = Dog {
        name: String::from("Max"),
        owner: &p1,
    };

    println!("{:?}", p1);
    println!("{:?}", d1);

    /*
     * LIFETIME ELISION: COMPILER BUILDS LIFETIMES FOR US WHEN EVIDENT
     */
    let mut a: &String;
    {
        let p2 = Person {
            name: String::from("Mary"),
        };
        // a = p2.get_name(); // ERROR: p2 does not live long enough
        a = p1.get_name();
    }

    println!("{}", a);

    /* 9.53. REFERENCE COUNTED VARIABLES => RC VARIABLES
     *
     * 1) A structure  that can hold multiple references to a variable
     * 2) Can be shared in different places
     *
     * Syntax:
     * use std::rc::Rc
     * fn do_smth(var: Rc<String>) ...
     * let var = Rc::new(String::from("test"));
     * var.clone()
     *
     * 3) Count the variable pointers
     *
     * Syntax:
     * Rc::strong_count(&var);
     *
     * RC VARIABLES USES REFERENCES TO MULTIPLE VARIABLES IN MUTIPLE PLACS WITHOUT THE NEED TO WORRY
     * ABOUT OWNERSHIP AND BORROWING.
     */

    let brand: Rc<String> = Rc::new(String::from("BMW"));
    println!("Pointers: {}", Rc::strong_count(&brand));
    {
        let car: Car = Car::new(brand.clone());
        car.drive();
        println!("Pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}", brand);
    println!("Pointers: {}", Rc::strong_count(&brand));
}

/*
 * FUCNTIONS
 */

// return a static string: Lifetime is end of program
fn get_str() -> &'static str {
    "Hello"
}

/*
 * STRUCTS
 */

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person, // MISSING A LIFETIME SPECIFIER - ADDED
}

struct Car {
    brand: Rc<String>,
}

/*
 * IMPLEMENTATIONS
 */

impl Person {
    // get name 1 and 2 is the same function. Compiler understands the lifetime and
    // you dont need to write it every time.
    fn get_name(&self) -> &String {
        &self.name
    }

    // fn get_name1<'l>(&'l self) -> &'l String {
    //     &self.name
    // }
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand: brand }
    }

    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}
