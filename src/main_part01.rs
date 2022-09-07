#![allow(unused)]

use std::io;
// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// ENUMS
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//CONSTANTS
const MAXIMUM_NUMBER: u8 = 4;

//STRUCTS
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

// TUPLE STRUCT
struct Colors(u8, u8, u8);

// IMPL KEYWORD
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    // #4 println!("Hello, world!");
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    // io:stdin().read_line();

    // #5 VARIABLE
    let mut x = 45; //i32
    println!("The value of x is {}", x);
    x = 60;
    println!("The value of x is {}", x);

    let y: i64 = 65; //i64 or u64
    let z: f32 = 6.7; //f32
    let b: bool = false; //bool

    // #6 IF STATEMENT
    let n = 60;
    if n < 45 {
        // has also other operands ==,!= etc.
        println!("The number is less than 30!");
    } else {
        // also has else if as usual
        println!("The number is greater than 30!");
    }

    // #7 LOOP
    let mut m = 0;
    loop {
        m += 1;
        if m > 3 {
            break;
        }
        println!("The LOOP value of n is {}", n);
    }

    // #8 WHILE LOOP
    let mut p = 1;
    while p <= 3 {
        // can use other operans suchas %
        println!("p WHILE LOOP is {}", p);
        p += 1;
    }

    // #9 FOR LOOP
    let numbers = 30..34;
    for i in numbers {
        println!("The FOR LOOP number is {}", i);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"]; // use .iter t access the vectors
    for (index, a) in animals.iter().enumerate() {
        println!(
            "The FOR LOOP VECTOR index is {} and the animal name is {}",
            index, a
        ); // in correct order
    }

    // #10 ENUMS
    // Express code is descriptive WAY

    let player_direction: Direction = Direction::Up; // : is for type :: is to access variants
    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are heading down!"),
        Direction::Left => println!("We are heading left!"),
        Direction::Right => println!("We are heading right!"),
    }

    // # 11 CONSTANT
    const MAXIMUM_NUMBER: u8 = 3;
    for q in 1..MAXIMUM_NUMBER {
        println!("{}", q);
    }

    // #12 TUPLES
    let tup1 = (20, 25, 30, 35, "Rust", 3.4, false, (1, 4, 7));
    println!("{}", tup1.3);
    println!("{}", (tup1.7).2);

    // #13 FUNCTIONS
    print_numbers_to(3);
    if is_even(30) {
        println!("It is even");

        // #14 CODE BLOCKS
        let a = 10;

        {
            // isolated
            let b = 5;
            println!("a: {} b:{}", a, b);
        }

        // #15 SDHADOWING
        let mut c = 10;
        {
            let c = 15;
        }
        let c = "C is a string";
        println!("c is {}", c);

        let c = true;
        println!("c is {}", c);

        //#16 REFERENCES
        let mut d = 10;
        let dr = &mut d;
        *dr += 1;
        println!("d is {}", dr);

        //#17 STRUCTS
        // color: red, green, blue
        let mut bg = Color {
            red: 255,
            green: 70,
            blue: 15,
        };
        println!("{}, {}, {}", bg.red, bg.green, bg.blue);

        //#18 TUPLE STRUCTS
        let mut reds = Colors(255, 0, 0);
        reds.2 = 60;
        println!("reds is {}, {}, {}", reds.0, reds.1, reds.2);

        //#19 PASS BYR REFERENCE
        let blue = Color {
            red: 0,
            green: 0,
            blue: 255,
        };
        print_color(&blue);

        //#20 ARRAYS
        let numbers = [1, 2, 3, 4, 5];
        for e in numbers.iter() {
            println!("{}", e);
        }
        for f in 0..numbers.len() {
            println!("{}", numbers[f]);
        }

        let numbers2: [i32; 5] = [1, 2, 3, 4, 5];
        for g in 0..numbers2.len() {
            println!("{}", numbers[g]);
        }

        let numbers3 = [2; 2];
        for j in 0..numbers3.len() {
            println!("{}", numbers3[j]);
        }

        //#21 IMPL KEY WORD
        // add methods to struct, more objects
        let my_rect = Rectangle {
            width: 10,
            height: 5,
        };
        my_rect.print_description();
        println!("Rectangle is a square: {}", my_rect.is_square());

        //#22 STRINGS
        let mut my_string = String::from("How is it going? ");
        println!("Length: {}", my_string.len());
        println!("String is empty? {}", my_string.is_empty());

        for token in my_string.split_whitespace() {
            println!("{}", token);
        }
        println!("Does the string contain 'it'? {}", my_string.contains("it"));
        my_string.push_str("Welcome to your class!.");
        println!("{}", my_string);
    }
}

// OUT OF MAIN FUNCCTION

fn print_numbers_to(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}
