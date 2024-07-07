//Write a function that removes the spaces from the string, then return the resultant string.
fn main() {
    no_space(String::from("Mack Octavian Amandus"));
}

fn no_space(x: String) -> String {
    x.split_whitespace().collect()
}

fn no_space2(s: String) -> String {
    s.replace(" ", "")
}
