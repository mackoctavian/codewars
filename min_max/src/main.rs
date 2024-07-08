fn main() {
    println!("Hello, world!");
}

//Find Maximum and Minimum Values of a List
fn minimum(arr: &[i32]) -> i32 {
    let min = arr.iter().min().unwrap();
    *min
}

fn maximum(arr: &[i32]) -> i32 {
    let mut max = 0;
    for i in arr.iter() {
        if i > &mut max {
            max = *i;
        }
    }

    max
}
