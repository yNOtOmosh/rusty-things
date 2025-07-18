fn main() {
    let some_number: Option<i32> = Some(0);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("The number is {}", n),
        None => println!("No number found"),
    }

    if no_number.is_none() {
        println!("Value is missing");
    }
}
