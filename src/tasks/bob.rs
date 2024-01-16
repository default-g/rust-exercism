pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",

        m if m.chars().last().unwrap() == '?'
                    && m.to_uppercase() == m
                    && m.chars().any(|c| c.is_alphabetic()) => "Calm down, I know what I'm doing!",

        m if m.chars().last().unwrap() == '?' => "Sure.",

        m if m.to_uppercase() == m
                    && m.chars().any(|c| c.is_alphabetic()) => "Whoa, chill out!",

        _ => "Whatever.",
    }
}
