fn main() {
    println!("Hello, world!");
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let total = s1 + s2 + s3;
    let average = total / 3;

    match average {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}
