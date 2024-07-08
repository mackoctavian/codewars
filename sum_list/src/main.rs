fn main() {
    let list = vec![10, 11, 50];
    println!("{}", sum(&list));
}

fn sum(list: &[i32]) -> i32 {
    let mut total = 0;
    for i in list {
        total += i;
    }
    total
}
