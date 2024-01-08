mod tasks;
use tasks::*;

fn main() -> () {
    let time = clock::Clock::new(12, 30);
    dbg!("{}",time);
}
