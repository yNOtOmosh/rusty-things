fn main() {
    let result = operate(2, 99, add);
    println!("Result: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn operate(a: i32, b: i32, f: fn(i32, i32) -> i32) -> i32 {
    f(a, b)
}