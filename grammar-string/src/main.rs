fn main() {
    let mut str = String::from("Hello, world");
    str.push('!');
    println!("{}", get_first(&str));
}

fn get_first(s: &str) -> &str {
    if s.len() == 0 {
        return "";
    } else {
        return &s[0..1];
    }
}
