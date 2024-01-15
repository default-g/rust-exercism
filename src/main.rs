mod tasks;
use tasks::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn add_1(mut a: i32) {
    a += 1;
}


fn main() -> () {
    println!("{}", beer_song::sing(1, 100));
}
