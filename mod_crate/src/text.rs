pub fn get_first(str: &str) -> &str {
    str.split_whitespace().next().unwrap()
}

pub fn get_first_char(str: &str) -> char {
    str.chars().next().unwrap()
}
