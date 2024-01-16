pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut proverb = Vec::new();

    for i in 0..(list.len() - 1) {
        proverb.push(format!("For want of a {} the {} was lost.", list[i], list[i + 1]));
    }
    proverb.push(format!("And all for the want of a {}.", list[0]));

    proverb.join("\n")
}
