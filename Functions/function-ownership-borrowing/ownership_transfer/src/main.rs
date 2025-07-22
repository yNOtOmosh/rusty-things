fn main() {
    let name = String::from("tony");
    takes_ownership(name); // name is moved
    // println!("{}", name); // error: value moved
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
