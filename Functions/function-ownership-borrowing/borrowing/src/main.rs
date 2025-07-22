fn main() {
    let name = String::from("Tony");
    borrows_data(&name); // pass by reference
    // println!("{}", name);
}

fn borrows_data(s: &String) {
    println!("{}", s);
}