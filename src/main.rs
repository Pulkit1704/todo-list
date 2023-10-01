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
}

fn update_task_status(task: &mut Task){
    task.update_status(); 
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

fn main() {
    let mut todo_list: Vec<Task> = Vec::new(); 

    // make it command line, use args: add,show, update, remove. 

    display_todo(&todo_list); 
    println!("after adding tasks"); 
    add_new_task(&mut todo_list, String::from("hello world"));
    add_new_task(&mut todo_list, String::from("hello world again")); 
    add_new_task(&mut todo_list, String::from("hello world again")); 
    add_new_task(&mut todo_list, String::from("hello world again")); 
    add_new_task(&mut todo_list, String::from("hello world again")); 
    display_todo(&todo_list); 

    println!("after update task status"); 
    update_task_status(&mut todo_list[0]); 

    display_todo(&todo_list); 

    println!("after removing task"); 
    remove_task(&mut todo_list, 3); 

    display_todo(&todo_list); 

}
