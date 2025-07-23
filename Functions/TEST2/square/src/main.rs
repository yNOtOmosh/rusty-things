fn main() {
    println!("squares from 1 to 10:");
    for n in 1..=10 {
        let result = square(n);
        println!("{}", result);
    }
}

fn square(n: i32) -> i32 {
    n * n
}
