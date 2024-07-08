fn main() {
    let num_list = vec![1, 2, 10, 56, 78, 100];
    let number = 110;

    println!("{}", find_num(&num_list, number));
}

fn find_num(list: &[i32], num: i32) -> bool {
    for i in list {
        if *i == num {
            return true;
        }
    }
    false
}
