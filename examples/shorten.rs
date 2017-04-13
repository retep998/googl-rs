// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
extern crate googl;

use std::env::{args};
use std::fs::{File};
use std::io::{Read};
use std::path::{Path};

fn main() {
    let longurl = args().nth(1).unwrap();
    let mut file = File::open(&Path::new("key.txt")).unwrap();
    let mut key = String::new();
    file.read_to_string(&mut key).unwrap();
    println!("{:?}", googl::shorten(&key, &longurl));
}
