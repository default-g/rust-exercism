pub fn is_valid(code: &str) -> bool {

    let stripped_string = code
        .chars()
        .filter(|s| !s.is_whitespace())
        .collect::<String>();

    if stripped_string.len() <= 1 {
        return false;
    }

    for symbol in stripped_string.chars() {
        if !symbol.is_digit(10) {
            return false;
        }
    }

    let mut int_vec: Vec<u32> = stripped_string
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    int_vec = int_vec.into_iter().rev().collect();
    let mut sum = 0;
    for i in 0..int_vec.len() {
        if i % 2 == 1 {
           int_vec[i] = int_vec[i] * 2;
           if int_vec[i] > 9 {
            int_vec[i] -= 9;
           }
        }
        sum += int_vec[i];
    }

    sum % 10 == 0
}

