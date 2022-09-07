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

    //#38 HTTP GET REQUEST
        
    let response_text = reqwest::get("http://youtube.local/hello")
        .expect("Couldnt make request")
        .text().expect("Could not read response text");
        println!("Response Text: {}", response_text);

    match reqwest::get("http://youtube.local/hello") {
        Ok(mut response) = {
            // Check if 200 Ok
            if response.status() = reqwest::StatusCode::Ok() {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text")
                }
        } else {
            println!("Response was not 200 OK");
            }
        }
            Err(_) => println!("Could not make request")
        }

    
}
