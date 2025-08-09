fn main() {
    for i in 1..=5 {
        println!("i = {}", i);
    }

    let items = ["apple", "banana", "mango"];

    for item in items.iter() {
        println!("{}", item);
    }
}
