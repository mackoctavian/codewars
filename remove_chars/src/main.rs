//Remove First and Last Character
fn main() {
    println!("{}", remove_char("Mack"));
}

fn remove_char(s: &str) -> String {
    let mut characters = s.chars();
    characters.next();
    characters.next_back();
    let word = characters.as_str();
    String::from(word)
}
