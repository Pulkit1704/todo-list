extern crate todo_list; 
use todo_list::Task; 

use std::io::Write;
use std::io::stdin; 
use std::io::stdout; 


fn runprompt(todo: &mut Vec<Task>){
    loop{
        let mut stdout = stdout(); 
        print!("> "); 
        stdout.flush().expect("can't flush the stdout"); 

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("cannot readline"); 

        // take the args into the run function of lib and get the result of the computation out. 
        // * what if the user enter a single command without any white spaces. 
        let args: Vec<&str> = buffer.split_whitespace().collect(); 

        todo_list::run(args, todo); 
        
    }
}

fn main() { 

    let mut todo: Vec<Task> = Vec::new(); 

    runprompt(&mut todo); 
    
}
