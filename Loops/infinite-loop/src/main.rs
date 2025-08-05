fn main() {
    let mut counter = 0;

    loop {
        println!("Counter: {}", counter);
        counter += 1;

        if counter == 6 {
            break;
        }
    }
}
