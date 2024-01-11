use std::collections::HashSet;

fn is_anagrams(word: &str, other_word: &str) -> bool {
    let mut string1: Vec<char> = word.to_lowercase().chars().collect();
    let mut string2: Vec<char> = other_word.to_lowercase().chars().collect();

    string1.sort();
    string2.sort();

    string1 == string2
}

#[allow(dead_code)]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut hash_set: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        if possible_anagram.to_lowercase() != word.to_lowercase() && is_anagrams(word, possible_anagram) {
            hash_set.insert(possible_anagram);
        }
    }

    return hash_set;
}
