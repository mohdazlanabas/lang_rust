#![allow(unused)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::process::Command;
mod decode;
extern crate regex;
use regex::Regex;
extern crate reqwest;
// extern crate rand;
// use rand::Rng;

fn main() {

    let mut cmd = Command::new("python");
    cmd.arg("test.py");
    // Execute Command
    match cmd.output() {
        Ok(o) => {
            unsafe {
            println!("Output: {}", String::from_utf8_unchecked(o.stdout));
        }
    },
        Err(e) => {
            println!("There was an error {}", e);
        }
    }
}
