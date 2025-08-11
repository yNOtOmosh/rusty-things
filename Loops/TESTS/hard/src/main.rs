fn main() {
    for i in 1..=200 {
        if i % 7 == 0 && i % 11 == 0 {
            println!("Found: {}", i);
            break;
        }
    }
}
