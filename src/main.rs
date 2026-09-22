fn main() {
    let t1 = String::from("Build AAS Tracker!");
    let n1 = 3;
    print_task_name(&t1); // pass as reference
    print_sub_tasks(n1); // n1 (i32) is copied

    println!("{t1}"); // can we print t1?
    println!("{n1}"); // can we print n1?
}


// Testing: Write a testing function that takes ownership of a String. 
// This string should represent your system's core resource (e.g., a ticket ID or a user message), 
// then write another testing function that works with an i32 (a Copy type), 
// representing a system parameter like a capacity limit or a status code. 
// Observe and document the difference in how Rust handles memory for these two types.

fn print_task_name(task_name: &String) {
    // is just reference t1
    println!("Your task is: {task_name}"); 

}

fn print_sub_tasks(num_sub_tasks: i32) { 
    // creates a copy of n1 when passed as argument 
    println!("This task has {num_sub_tasks} sub tasks!"); 

    // nothing is returned!
}
