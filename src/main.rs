mod tasks;
use tasks::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> () {
    let input = &["4S 5H 4C 8D 4H", "10D JH QS KD AC"];
    let output = poker::winning_hands(input).into_iter().collect::<HashSet<_>>();
    let expected = ["10D JH QS KD AC"].into_iter().collect::<HashSet<_>>();
}
