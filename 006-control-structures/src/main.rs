#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![warn(non_snake_case)]

use rand::{prelude::ThreadRng, Rng};
use Suit::{Club, Diamond, Heart, Spade};

fn main() {
    // If statements
    // --------------------
    let mut rng: ThreadRng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..11);

    if num >= 5 {
        println!("Number {} is greater than or equal to 5", num);
    } else {
        println!("Number  {} is smaller than 5", num);
    }

    let res: bool = if num >= 5 { true } else { false };
    println!("{}", res);

    // Match statement
    // -----------------------
    print_choice(Heart);
    print_choice(Club);
    print_choice(Diamond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);

    // Pattern matching
    // ----------------------

    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    for x in 0..3 {
        for y in 0..3 {
            match_points(x, y);
        }
    }

    // For loop
    // ----------------

    let pets = ["cat", "dog", "chihiahua", "hamster", "bear"];

    for pet in pets.iter() {
        if pet == &"chihiahua" {
            println!("{} barks too much", pet);
            continue;
        }

        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break;
        }

        println!("I love any {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square)
    }

    // While loop
    // -------------------
    get_squares(100);
    get_cubes(100);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Heart => {
            println!("\u{2665}")
        }
        Suit::Spade => {
            println!("\u{2660}")
        }
        Suit::Club => {
            println!("\u{2663}")
        }
        Suit::Diamond => {
            println!("\u{2666}")
        }
    }
}

fn country(code: i32) {
    let country: &str = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "Unknown",
        _ => "Invalid",
    };
    println!("Country is {}", country);
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "No",
        1 | 2 => "One or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "An even amount",
        _ => "Lots of",
    };
}

fn match_points(x: i32, y: i32) {
    let point: (i32, i32) = (x, y);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("{} {}", x, y),
    }
}

fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0}, * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1} ", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break;
        }
    }
}
