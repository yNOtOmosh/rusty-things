fn main() {
    let greeting: &str = "Hello, dunia!"; // string slice, immutable

    let mut name = String::from("tony"); // growable string
    name.push_str(" Omondi");

    println!("{}", greeting);
    println!("Full name: {}", name);
}
