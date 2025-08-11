fn main() {
    let mut sum = 0;
    let mut i = 1;

    loop {
        if i > 100 {
            break;
        }

        if i % 2 == 0 {
            sum += i;
        }

        i += 1;
    }
    println!("Sum of even numbers: {}", sum);
}
