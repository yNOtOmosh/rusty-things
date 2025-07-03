/* fn main() {
    println!("Hello, world!");
}*/

fn main() {
    let x = 5; //immutable
    let mut y = 10; //mutable

    println!("x = {}, y ={}", x, y);

    y += 5; // allowed cz y is mutable
    println!("Now y = {}", y);
}