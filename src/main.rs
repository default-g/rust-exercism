mod tasks;
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
    forth.eval(": a 1 2 over ;");
    forth.eval(": a 1 ;");
    forth.eval("a");
    for item in forth.stack() {
        print!("{} ", item);
    }
}
