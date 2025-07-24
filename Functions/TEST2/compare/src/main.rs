fn compare (a: i32, b: i32) -> &'static str {
    if a > b {
        "greater"
    } else if a < b {
        "less"
    } else {
        "equal"
    }
}

fn main() {
    println!("{}", compare(5, 3));
    println!("{}", compare(9, 3));
    println!("{}", compare(4, 4));
}