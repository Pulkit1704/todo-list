use std::env; 

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
        println!("id: {}, name: {}, done: {}", item.id, item.task, item.done_status);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_string: String){
    
    let id_no: u64 =( todo_list.len() + 1).try_into().unwrap(); 

    let task: Task = Task{
        task: task_string.clone(), 
        done_status: false, 
        id: id_no, 
    };

    todo_list.push(task); 

    println!("{} added to the todo list", task_string); 
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64){

   for index in 1..todo_list.len(){

        if todo_list[index].id == id_no{
            todo_list.remove(index); 
            break; 
        }
   }
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

fn parse_arguments(args: &[String], todo_list: &mut Vec<Task>){
    let command = args[1].as_str();

    match command{
        "add" => {

            let new_task = args[2].clone(); 

            add_new_task(todo_list, new_task); 
            display_todo(todo_list); 
        },

        "show" =>{

            display_todo(todo_list); 

        },

        "delete" => {
            let task_id: u64 = args[2].parse().unwrap(); 

            remove_task(todo_list, task_id); 
        },

        "update" => {
            let task_id: u64= args[2].parse().unwrap();

            let new_task = args[3].clone(); 

            let old_task = get_task(todo_list, task_id).unwrap(); 
            old_task.update_task(new_task);  

        },

        "done" => {
            let task_id: u64 = args[2].parse().unwrap(); 

            let done_task = get_task(todo_list, task_id).unwrap(); 
            done_task.update_status();
        }

        _ => {
            println!("invalid arguments");
        }
        
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new(); 

    let args: Vec<String> = env::args().collect(); 

    parse_arguments(&args, &mut todo_list);

}
