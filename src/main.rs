mod tasks;
use std::collections::HashMap;
use std::collections::HashSet;
use tasks::forth::Forth;
use tasks::*;

use std::fmt;

fn add_1(mut a: i32) {
    a += 1;
}

fn main() -> () {
    let mut forth = Forth::new();

    forth.eval("1 2 * 3 / Dup dup dup");
    for item in forth.stack() {
        print!("{} ", item);
    }
}
