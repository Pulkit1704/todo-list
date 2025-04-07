extern crate todo_list; 
use todo_list::Task; 

use std::io::Write;
use std::io::stdin; 
use std::io::stdout; 

fn runprompt(todo: &mut Vec<Task>){
    loop{
        let mut stdout = stdout().lock(); 
        print!("(todo list) > "); 
        stdout.flush().expect("can't flush the stdout"); 

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("cannot readline"); 

        // take the args into the run function of lib and get the result of the computation out. 
        let command: &str = buffer.split_whitespace().collect::<Vec<&str>>()[0];  
        let task_string = concatenate_task_string(buffer.split_whitespace().collect());  

        let args = vec![command, task_string.as_str()]; 

        todo_list::run(args, todo); 
        
    }
}

fn concatenate_task_string(word_vec: Vec<&str>)-> String{

    let mut result = String::new(); 

    // we are assuming that everything after the first word is part of the task string. 
    for i in 1..word_vec.len(){
        result = result + word_vec[i]; 
        result = result + " "; 
    }

    result 
}

fn main() { 

    let mut todo: Vec<Task> = Vec::new(); 

    runprompt(&mut todo); 
    
}
