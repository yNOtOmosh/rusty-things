fn main() {
    let big_number: i64 = 10000;
    println!("big_number: {}", big_number);

    let price: f32 = 82.50;
    println!("price is: ${}", price);

    let is_valid: bool = true;
    println!("valid: {}", is_valid);

    let grade: char = 'A';
    println!("grade: {}", grade);

    let person: (&str, i64) = ("Tony", 25);
    println!("name: {}", person.0);
    println!("age: {}", person.1);

    let numbers: [i32; 5] = [2, 4, 6, 8, 10];
    for numbers in numbers.iter () {
        println!("{}", numbers);
    }
}
