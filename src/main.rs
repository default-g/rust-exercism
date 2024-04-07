mod tasks;
use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdout;
use std::io::Write;
use tasks::forth::Forth;
use tasks::*;

use std::fmt;

fn add_1(mut a: i32) {
    a += 1;
}

fn main() -> () {
    let mut forth = Forth::new();

    let result = forth.eval("6 3");

    if result.is_err() {
        panic!("ERROR");
    }
    for value in forth.stack() {
        print!("{} ", value);
    }
}
