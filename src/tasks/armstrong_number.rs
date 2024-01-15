pub fn is_armstrong_number(num: u32) -> bool {
    num.to_string()
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|value| value.to_digit(10).unwrap())
        .map(|value: u32| u128::pow(value as u128, num.to_string().len() as u32))
        .sum::<u128>() == num as u128
}
