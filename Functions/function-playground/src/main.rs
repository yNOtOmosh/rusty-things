use std::io;

fn main() {
    loop {
        println("\n--- the playground ---");
        println("uno. greet");
        println("dos. square numbers");
        println("tres. compare");
        println("cuatro. string length");
        println("cinco. multiply using closure");
        println("cero. exit");

        let choice = read_input("enter your choice: ");

        match choice.trim() {
            "uno" => {
                let name = read_input("Enter name: ");
                greet(name.trim());
            }
            "dos" => {
                for i in 1..= 10 {
                    println!("{} squared = {}", i, square(i));
                }
            }
            "tres" => {
                let a = read_input("Enter first number: ").trim().parse::<i32>().unwrap();
                let b = read_input("Enter second number: ").trim().parse::<i32>().unwrap();
                println!("Result: {}", compare(a, b));
            }
            "cuatro" => {
                let input = read_input("Enter a string: ");
                println!("Length = {}", string_length(input.trim()));
            }
            "cinco" => {
                let a = read_input("Enter first number: ").trim().parse::<i32>().unwrap();
                let b = read_input("Enter second number: ").trim().parse::<i32>().unwrap();
                let multiply = |x: i32, y: i32| x * y;
                println!("{} x {} = {}", a, b, multiply(a, b));
            }
            "cero" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again!"),
        }
    }
}

// function definations