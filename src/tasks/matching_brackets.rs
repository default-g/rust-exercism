use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];


    let brackets_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let is_closing_bracket = |char| {
        brackets_map.values().find(|&key| key == &char).is_some()
    };

    let is_opening_bracket = |char| {
        brackets_map.keys().find(|&key| key == &char).is_some()
    };

    for char in string.chars() {
        if is_opening_bracket(char) {
            stack.push(char);
        }

        if is_closing_bracket(char) {

            if stack.is_empty() {
                return false;
            }


            let top = &stack.pop().unwrap();
            if char != *brackets_map.get(&top).unwrap() {
                return false;
            }
        }

        dbg!(&stack);

    }

    stack.is_empty()
}
