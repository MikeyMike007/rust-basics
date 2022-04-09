// Global variabes - NOT SAFE
// ------------------------------
static mut R: i32 = 0;

// Macro
// ------------------------------
macro_rules! my_macro {
    () => {
        println!("First macro");
    };
}
macro_rules! name {
    ($name: expr) => {
        println!("Hey {}", $name);
    };
}

macro_rules! name2  {
    ($($name2: expr),*) => ($(println!("Hey {}", $name2);)* )

}

macro_rules! xy {
    (x => $e: expr) => {
        println!("X is {}", $e)
    };
    (y => $e: expr) => {
        println!("Y is {}", $e)
    };
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    };
}

fn main() {
    // Functions - pass by reference
    // ---------------------------------------
    let mut name: &str = "John";
    println!("Hi, my name is {}", &name);
    change_name(&mut name);
    println!("Hi, my name is {}", &name);

    // Scope
    {
        let a = 3;
        println!("a = {}", a);
    }

    // println!("a = {}", a); // ERROR OUT OF SCOPE

    // Need to use unsafe{} to access global variables
    unsafe {
        R = 4;
        println!("R = {}", R);
    }

    // Closures - Functons in functions
    // -----------------------------------

    // Anonymous (lambda function)
    let a = |a: i32| a + 1;
    println!("{}", a(6));

    // Function assigned to a variable
    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };

    println!("{}", b(6));

    // generic clojure
    let gen = |x| println!("Recieved = {}", x);
    let gen1 = |x| println!("Recieved = {}", x);
    gen(3);
    // gen(true); // ERROR CANNOT CHANGE TYPE OF x
    gen1(true);

    // Higher order functions
    // ---------------------
    let square = |a: i32| a + a;
    apply(square, 10);

    // Calculate the sumof all the squares less than 500 - without higher order function
    let limit: i32 = 500;
    let mut sum: i32 = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum is {}", sum);

    // Calculate the sumof all the squares less than 500 - with higher order function
    let sum2 = (0..)
        .map(|x: i32| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x: &i32| is_even(*x))
        .fold(0, |sum: i32, x: i32| sum + x);

    println!("Sum2 = {}", sum2);

    // Macros - meta programming = Write code that writes code
    my_macro!();
    name!("John");
    name2!("Alex", "Mary", "Carol");
    xy!(x => 5);
    xy!(y => 3 * 9);
    build_fn!(hey);
    hey();
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}
fn change_name(from: &mut &str) {
    *from = "Anders";
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}
