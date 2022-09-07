#![allow(unused)]
#![allow(dead_code)]

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
extern crate reqwest;
// extern crate rand;
// use rand::Rng;

fn main() {

    //#37 OPTION (ENUM)

    let name = String::from("azlan");
    println!("Character at index 8: {}", match name.chars().nth(8) {
        Some(c) => c.to_string(),
        None => "No character at index 8".to_string()
    });
    
    println!("Occupation is {}", match get_occupation("Azlan") {
        Some(o) => o,
        None => "No aoccupation found"
    });

}

    fn get_occupation(name: &str) -> Option<&str> {
        match name {
            "Azlan" => Some("Software Developer"),
            "Mike" => Some("Dentist"),
            _ => None
        }
    
}
