fn main() {
    greet("tony"); //call the greet function
}

// define the greet function outside of main
fn greet(name: &str) {
    println!("hello, {}!", name);
}
