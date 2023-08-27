extern crate ansi_term;

use std::io;
use std::fs::File;
use std::io::{Write, Read};
use ansi_term::{Colour, Style};


struct Note {
    text: String,
    active: bool,
}

fn print_all_todo(v: &Vec<Note>){ 
    let style_TODO = Colour::Fixed(156);
    let style_done = Colour::Fixed(40);
    println!("{}", style_TODO.paint("TODO:"));
    for (i, el) in v.iter().enumerate(){
        if el.active == true{println!("{}) {}", i+1, el.text);}
        else {println!("{}) {}", i+1, style_done.paint(el.text.as_str()));}
    }
    if v.len() == 0 {println!("There are no tasks.")};
}

fn enter_something() -> String{
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    command.trim().to_string()
}

fn enter_task_text() -> String{
    println!("Enter a new task text:");
    enter_something()
}

fn enter_task_number(max_number:usize) -> usize{
    let mut a = true;
    let mut task_number_s = String::new();
    let mut task_number:usize = 0;
    while a{
        println!("Enter the task number:");
        task_number_s = enter_something();
        task_number =  match task_number_s.parse() {
            Ok(number) => number,
            Err(error) => {println!("Try again"); continue;},
        };
        if task_number < 1 || task_number > max_number {println!("Try again"); continue;}
        a = false;
    }
    task_number-1
}

fn vec_read(path:&str) -> Vec<Note>{
    let mut vec:Vec<Note> = Vec::new();

    let mut data_file = File::open(path).expect("open failed");
    let mut content = String::new();
    data_file.read_to_string(&mut content).expect("read failed");

    let mut num_l:usize = 0;
    for line in content.lines() {
        if num_l == 0{
            let len_todolist:usize = line.trim().parse().unwrap();
            for _i in 0..len_todolist{
                let n = Note{
                    text: String::new(),
                    active: true,
                    };
                vec.push(n);
            }
        }else if (num_l % 2) == 1{
            let n = (num_l-1)/2;
            vec[n].text = line.to_string();
        } else {
            let n = (num_l-1)/2;
            if line.trim() == "true" {vec[n].active = true;}
            else {vec[n].active = false;}
        }
        num_l = num_l + 1;
    }
    vec
}

fn vec_write(v:Vec<Note>, path:&str){
    let mut data_file = File::create(path).expect("creation failed");

    data_file.write((v.len().to_string()+"\n").as_bytes()).expect("write failed");
    for el in v{
        data_file.write((el.text+"\n").as_bytes()).expect("write failed");

        if el.active == true {
            data_file.write("true\n".as_bytes()).expect("write failed");
        } else {
            data_file.write("false\n".as_bytes()).expect("write failed");
        }
        
    }
}

fn main() {
    let command_style = Colour::Fixed(73);

    let path = "todo.txt";
    let mut todo_list: Vec<Note> = vec_read(path);

    loop{
        print_all_todo(&todo_list);

        println!("
Enter the command number:
1. Create a task
2. Change an existing task
3. Delete the task
4. Complete(or not complete) the task
5. Delete all completed tasks
0. Exit");

        let command = enter_something();
        
        if command == "1" {
            let new_todo = enter_task_text();

            let todo_note = Note{
                text: new_todo,
                active: true,
            };

            todo_list.push(todo_note);
        } 
        else if command == "2" {
            if todo_list.len() == 0{
                println!("There are no tasks. Add at least one task and try again.");
                continue;
            }
            
            let task_number:usize = enter_task_number(todo_list.len());
            todo_list[task_number].text = enter_task_text();
        } 
        else if command == "3" {
            if todo_list.len() == 0{
                println!("There are no tasks. Add at least one task and try again.");
                continue;
            }

            let task_number:usize = enter_task_number(todo_list.len());
            todo_list.remove(task_number);
        }
        else if command == "4" {
            if todo_list.len() == 0{
                println!("There are no tasks. Add at least one task and try again.");
                continue;
            }

            let task_number:usize = enter_task_number(todo_list.len());
            todo_list[task_number].active = !todo_list[task_number].active;
        }
        else if command == "5" {
            let mut number = 0;
            for _el in 0..todo_list.len() {
                if todo_list[number].active == false {
                    todo_list.remove(number);
                } else {number = number + 1;}
            }
        } 
        else if command == "0" {
            let style_exit = Colour::Fixed(221);
            println!("{}", style_exit.paint("Have a productive day!"));
            vec_write(todo_list, path);
            break
        } 
        else {
            println!("Invalid command number, try again, remember, the command number contains one digit");
        }
    }
}
