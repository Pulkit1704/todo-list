use std::process;

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

// return a result type with empty function and a string message. 
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


fn display_help(){
    let help: &str = "
        Welcome to the todo_list application. 
        structure of query: 
            command arguments 

        supported commands: 

        add - Add a new task to the todo list, followed by a new task string. The task string should NOT be space separated. 

        show - Display the todo list 

        delete - delete a task from the todo list, followed by an integer number task id. 

        update - change the name of a task, followed by an integer number task id. 

        done - change the done status of a task from false to true, follwed by an integer number task id. 

        exit- exit the program. 

        help - display this help message. 
    ";

    println!("{}", help); 

}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>){
    let command = args[0];

    match command{
        "add" => {

            if let Some(value) = args.get(1){
                let new_task = *value; 
                add_new_task(todo_list, new_task); 
                display_todo(todo_list); 
            }else{
                println!("please provide a new name for the task"); 
            }
        },

        "show" =>{

            display_todo(todo_list); 

        },

        // integer parsing needs error handling 

        "delete" => {

            match &args[1].parse::<u64>(){
                Ok(value) =>{
                    remove_task(todo_list, *value);
                }

                Err(message) =>{
                    println!("{}", message.to_string()); 
                }
            }
        },

        "update" => {
            
            // possibility 1: id parsing error 
            match &args[1].parse::<u64>(){
                Ok(value) => {

                    // possibility 2: task getting error 
                    if let Ok(task) = get_task(todo_list, *value){

                        // possibility 3: no third argument provided. 
                        if let Some(value) = args.get(2){
                            let new_task = *value; 
                            task.update_task(new_task.into()); 
                        }else{
                            println!("no new task provided"); 
                        }

                    }else{
                        println!("task not found in todo list"); 
                    }
                },

                Err(message) => {
                    print!("{}", message); 
                }
            }
        },

        "done" => {

            match &args[1].parse::<u64>(){
                Ok(value) =>{
                    if let Ok(task) = get_task(todo_list, *value){
                        task.update_status(); 
                    }else{
                        println!("task id not found in list"); 
                    }
                }, 
                Err(message) =>{
                    println!("{}", message.to_string());
                }
            }
        },

        "exit" => {
            process::exit(0); 
        }

        "help" | _ => {
            display_help(); 
        }
        
    }
}


pub fn run(args: Vec<&str>, todo: &mut Vec<Task>){

    parse_arguments(args, todo); 
        
}
