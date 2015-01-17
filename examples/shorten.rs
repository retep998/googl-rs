// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>

#![allow(unstable)]

extern crate googl;

use std::io::fs::File;
use std::os::args;

fn main() {
    let arg = args();
    let longurl = &*arg[1];
    let mut file = File::open(&Path::new("key.txt")).unwrap();
    let key = file.read_to_string().unwrap();
    println!("{:?}", googl::shorten(&*key, &*longurl));
}
