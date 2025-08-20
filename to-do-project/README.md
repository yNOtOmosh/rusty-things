Features for MVP
minimum features:
1. add a task
2. list a task
3. remove a task by index

optional :
1. mark a task as complete
2. save tasks between runs(to a text or JSON file)

Data structure:
-store task as a strut
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
description: String,
completed: bool,
}
-keep all tasks in:
let mut tasks: Vec<Task> = Vec::new();

-Persistent storage:
store tasks in a JSON file
1. load tasks at program start
2. save tasks after every change

-cli menu loop(suggested)
loop {
println!("1) Add Task\n2) View Tasks\n3) Remove Task\n4) Quit");
// Read user choice
// Match on choice, run function
}

-implementation flow
functions:
1. load_tasks() → read JSON into Vec<Task>.
2. save_tasks() → write JSON from Vec<Task>.
3. add_task() → ask user for task description, push to list, save.
4. list_tasks() → print all tasks with index & status.
5. remove_task() → ask for index, remove, save.

-other improvements
Command-line arguments instead of interactive menu (todo add "Buy milk").
Colorized output (using colored crate).
Deadlines & sorting.