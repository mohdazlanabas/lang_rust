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

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
        }
    }
}
fn main() {

            //#39 ENUM METHODS
    let d = Day::Tuesday;
    println!("Is d a weekday? {}", d.is_weekday());
    
    
}
