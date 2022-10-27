// 1.   Create a Todo App that implements Trait, Struct, Option, Error, Result.
// 2.   We should be able to enter a value on the terminal.
// 3.   Current value and previous values should be return when we hit ENTER key.

use std::io::{self, Read, Write};
use std::vec::Vec;

struct TodoApp {
    pub list: Vec<String>,
}

pub trait ReadLine {
    fn read_line(&mut self) -> Result<usize, ()>;
}

impl ReadLine for TodoApp {
    fn read_line(&mut self) -> Result<usize, ()> {
        print!("Enter a new value: ");
        io::stdout().flush().unwrap();

        let mut buff = String::new();
        let stdin = io::stdin();

        let size = stdin.read_line(&mut buff);
        // buff.truncate(size - 2);
        match size {
            Ok(val) => {
                buff.truncate(val - 2);
                self.list.push(buff.clone());
                return Ok(buff.len())
            },
            Err(_) => return Err(())
        }
    }
}

impl TodoApp{
    fn print_list(&self){
        println!("Returns: {:?}", self.list);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let mut test_input: String = String::new();
    let mut todo_list: Vec<String> = vec![];
    let mut todo_app: TodoApp = TodoApp { list: vec![]};

    // let stdin = io::stdin();
    loop {
        // read input
        todo_app.read_line().unwrap();

        // print out the list
        todo_app.print_list();
    }
}
