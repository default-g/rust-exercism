pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|char: char| char.is_whitespace() || char == '-' || char == '_')
        .flat_map(|str| {
            str.chars().take(1).chain(
                str.chars()
                    .skip_while(|char| char.is_uppercase())
                    .filter(|char| char.is_uppercase())
            )
        })
        .collect::<String>()
        .to_uppercase()
}
