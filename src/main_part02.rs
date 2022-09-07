#![allow(unused)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, ErrorKind, Write};
mod decode;
extern crate regex;
use regex::Regex;
// extern crate rand;
// use rand::Rng;

struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
}

//DEFINNNG TRAIT
struct Person2 {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    // Speak
    fn speak(&self);
    // Check If Can Speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person2 {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    //#23 IMPLEMENTING TRAITS
    let dom = Person {
        name: String::from("Domenic"),
        age: 21,
    };
    println!("{}", dom.to_string());

    //#24 VECTORS
    // Array on steroid
    // let my_vector: Vec<i32> = Vec:new();\
    let mut my_vector = vec![1, 2, 3, 4, 5];
    my_vector.push(49);
    my_vector.remove(1);
    println!("{}", my_vector[2]);
    for number in my_vector.iter() {
        println!("{}", number);
    }

    //#25 READING A FILE
    let mut file = File::open("info.txt").expect("Cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cant read the file");
    println!("File contents: \n{}", contents);

    //#26 Command Line
    let args: Vec<String> = env::args().collect();
    for argument in args.iter() {
        println!("{}", argument)
    }

    //#27 WRITING TO A FILE
    let mut file = File::create("output.txt").expect("Cant read the file");
    file.write_all(b"Welcome To Rust")
        .expect("Cant write the file");

    //#28 DEFINING TRAITS
    let person2 = Person2 {
        name: String::from("Bob"),
        age: 41,
    };
    println!("Can {} speak? {}", person2.name, person2.can_speak());

    //#29 VECTORS
    let number2 = 15;
    match number2 {
        1 => println!("It is one "),
        2..=20 => println!("It is greater than one"),
        _ => println!("It does not match"),
    }

    let name3 = "Azlan";
    match name3 {
        "Azlan" => println!("It is the one"),
        "Domenic" => println!("It is not the "),
        _ => println!("It does not match"),
    }

    /*
    //#30 USER INPUT
    let mut input = String::new();
    println!("Hey say something: ");
    match  io::std().read_line(&mut_input) {
        Ok(_) => {
            println!("Success! You said: {}", input.to.uppercase()),
        },
        Err(e) => println!("Error: {}", e),
    }
    */

    //#31 HASHMAP
    let mut marks = HashMap::new();
    // Add Values
    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 90);
    marks.insert("UX Design", 85);
    marks.insert("Computer c50", 80);
    // find length of HashMap
    println!("How many subjects ? {}", marks.len());

    //Get a single value
    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Development", mark),
        None => println!("No Web Development found"),
    }

    // Remove a value
    marks.remove("UX Design");
    // Loop Through Hash Map
    for (subject, marks) in marks {
        println!("For {} you got {}%", subject, marks);
    }

    //#32 RANDOM NUMBERS
    // Skip as issues with Rand

        //#33 MORE STRINGS METHOO
    // Replace
    {
        let my_string = String::from("Rust is great");
        println!("After replace: {}", my_string.replace("great", "fantastic"));
    }
    // Lines
    {
        let my_string = String::from("The weather is \n nice \n outside ");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    // Split
    {
        let my_string = String::from("Eat+Nase+Goreng+This+Morning");
        let tokens: Vec<&str> = my_string.split("+").collect();
        println!("{}", my_string);
        println!("Ar index 2: {}", tokens[2]);
    }
    //Trim

    {
        let my_string = String::from("     My name is Azlan     \n \r ");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    // Chars
    {
        let my_string = String::from(" My name is Azlan");
        println!(" {} ", my_string);
        match my_string.chars().nth(4) {
            Some(c) => println!("Characters at index 4: {}", c),
            None => println!("No Character at index 4"),
        }
    }
    
    //#34 MULTIPLE SOURCE FILES
    decode::print_message();
    
    //#35 REGULAR XPRESSIONS
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "decode";
    println!("Found match {}", re.is_match(text));

    //#36 MODULES
    // same as ??#34 source files on Mod

    //#37 OPTION (ENUM)
    

}
