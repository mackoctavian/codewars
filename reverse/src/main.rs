fn main() {
    let name = "mack";

    //Reversing a string
    let characters: String = name.chars().rev().collect();

    println!("{:?}", characters);
}
