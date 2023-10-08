// use std::fs::File; 
// use std::io::prelude::*; 

#[derive(Debug)]
pub struct Task{
    task: String,
    done_status: bool, 
    id: u64, 
}

impl Task{
    fn update_status(&mut self){
        self.done_status = true; 
    }

    fn update_task(&mut self, new_name: String){
        self.task = new_name; 
    }
}

fn display_todo(todo_list: &Vec<Task>){
    if todo_list.len() < 1 {
        println!("Empty todo list"); 
        return; 
    }

    for item in todo_list{
        println!("{}, name: {}, done: {}", item.id, item.task, item.done_status);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str){
    
    let id_no: u64 =( todo_list.len() + 1).try_into().unwrap(); 

    let task: Task = Task{
        task: task_string.into(), 
        done_status: false, 
        id: id_no, 
    };

    todo_list.push(task); 

    println!("{} added to the todo list", task_string); 
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64){

    todo_list.retain(|task| task.id != id_no); 

}

fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str>{

    for task in todo_list{
        if task.id == task_id{
            return Ok(task);
        }else{
            continue;
        }
    };

    return Err("Task not found in todo list"); 

}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>){
    let command = args[0];

    match command{
        "add" => {

            let new_task = args[1].clone(); 

            add_new_task(todo_list, new_task); 
            display_todo(todo_list); 
        },

        "show" =>{

            display_todo(todo_list); 

        },

        // integer parsing needs error handling 

        "delete" => {
            let task_id: u64 = args[1].parse::<u64>().unwrap(); 

            remove_task(todo_list, task_id); 
        },

        "update" => {
            let task_id: u64= args[1].parse::<u64>().unwrap();

            let new_task = args[2].clone(); 

            let old_task = get_task(todo_list, task_id).unwrap(); 
            old_task.update_task(new_task.into());  

        },

        "done" => {
            let task_id: u64 = args[1].parse::<u64>().unwrap(); 

            let done_task = get_task(todo_list, task_id).unwrap(); 
            done_task.update_status();
        },

        _ => {
            println!("invalid arguments");
        }
        
    }
}


pub fn run(args: Vec<&str>, todo: &mut Vec<Task>){

    parse_arguments(args, todo); 
        
}
