use std::collections::HashMap;
use std::thread;
use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut real_worker_count = worker_count;
    let hash_map: Mutex<HashMap<char, usize>> = Mutex::new(HashMap::new());

    if real_worker_count > input.len() {
        real_worker_count = input.len();
    }

    if input.is_empty() {
        return hash_map.lock().unwrap().clone();
    }

    let items_per_worker: usize = input.len() / real_worker_count;

    for slice in input.chunks(items_per_worker) {
        thread::scope(|scope| {
            scope.spawn(|| {
                for string in slice {
                    for symbol in string.chars()
                        .filter(|symbol| symbol.is_alphabetic())
                        .map(|symbol| symbol.to_ascii_lowercase()) {
                            *hash_map.lock().unwrap()
                                .entry(symbol)
                                .or_default() += 1;
                    }
                }
            });
        });
    }

    let final_map = hash_map.lock().unwrap();
    final_map.clone()
}
