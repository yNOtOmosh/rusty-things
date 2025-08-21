use std::io::{self, Write};

struct Task {
    description: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n** to- do **");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Tasks");
        println!("4. Quit");
        print!("> ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => add_task(&mut tasks)?,
            "2" => list_tasks(&tasks)?,
            "3" => remove_task(&mut tasks)?,
            "4" => break,
            _ => eprintln!("Invalid choice"),
        }
    }

    Ok(())
}


