fn main() {
    let person: (i32, &str, f64) = (25, "tony", 72.5);

    println!("Age: {}", person.0);
    println!("Name: {}", person.1);
    println!("weight: {}kg", person.2);

    let (age, name, weight) = person;
    println!("{} is {} years of age and weighs {}kg", name, age, weight);
}
