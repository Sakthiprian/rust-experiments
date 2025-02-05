use std::{fs, process::exit};
use serde::{Serialize,Deserialize};
use std::io;

#[derive(Debug,Serialize,Deserialize)]
enum State {
    Completed,
    Pending,
    Ongoing,
}

#[derive(Debug,Serialize,Deserialize)]
struct Task {
    id: usize,
    name: String,
    status: State 
}

impl Task{
    fn update_state(&mut self, new_state: State){
        self.status = new_state;
    }
}

fn add_task(task_name: String, task_list: &mut Vec<Task>){

    let length = task_list.len();
    let task = Task{
        id: length,
        name: task_name,
        status: State::Pending,
    };

    task_list.push(task);
}

fn remove_task(id: usize, task_list: &mut Vec<Task>){
    task_list.retain(|task| task.id != id);
}

fn list_tasks(tasks:&mut Vec<Task>){
    for (i,task) in tasks.iter().enumerate(){
        println!("{}. {:?} {:?}",i+1, task.name, task.status);
    }
}

fn read_file_and_load_data( path:String)->Vec<Task>{
    /*
    Function should ideally read a file and load up the tasks vector
    */
    //deserialize json to get back tasks
    let content: String = fs::read_to_string(path).expect("Unable to read the file");
    let deserialized:Vec<Task> = serde_json::from_str(&content).expect("Unable to deserialize string");
    deserialized
}

fn write_current_state_to_file(tasks: &Vec<Task>, task_file_path: String){
    /*
    Function should ideally write the current todo list state everytime a change is made to the tasks
     */

     let json_string = serde_json::to_string(&tasks).expect("Unable to serialize to json");
     println!("Serialized tasklist: {:?}", json_string);

     fs::write(task_file_path, json_string).expect("Unable to write file");
}

fn add_task_action(task_list: &mut Vec<Task>){
    println!("Enter the task name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Enter a valid name");

    add_task(name, task_list);
}


fn remove_task_action(task_list: &mut Vec<Task>){
    println!("Enter the task id: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Enter a valid number");
    let id:i32 = id.trim().parse().expect("Enter a valid number");
    for (index, task) in task_list.iter().enumerate(){
        if task.id== id as usize{
            remove_task(index, task_list);
            break;
        }
    }   
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let task_file_path = "tasks.json";

    //Build a menu driven UI first

    println!("Menu:");
    println!("1. Add Task");
    println!("2. Remove Task");
    println!("3. Show Tasks");
    println!("4. Exit Program");
    println!("Enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to read line");

    let choice:i32 = choice.trim().parse().expect("Not a number");

    match choice{
        1 => add_task_action(&mut tasks),
        2 => remove_task_action(&mut tasks),
        3 => list_tasks(&mut tasks),
        4 => exit(0),
        _ => println!("Not a valid choice")
    };

    println!("The choice is : {} ", choice)
}
