fn main() {
    say_hi(Some("Tony"));
    say_hi(None);
}

fn say_hi(name: Option<&str>) {
    match name {
        Some(n) => println!("hi, {}!", n),
        None => println!("hi, stranger!"),
    }
}
