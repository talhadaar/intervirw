// 1.   Create a Todo App that implements Trait, Struct, Option, Error, Result.
// 2.   We should be able to enter a value on the terminal.
// 3.   Current value and previous values should be return when we hit ENTER key.

use std::io::{self, Write};
use std::vec::Vec;

struct TodoApp {
    pub list: Vec<String>,
}

pub trait ReadLine {
    fn read_line(&mut self) -> Option<usize>;
}

impl ReadLine for TodoApp {
    fn read_line(&mut self) -> Option<usize> {
        // get the input and flush
        print!("Enter a new value: ");
        io::stdout().flush().unwrap();

        // make temporary buffer for StdIn buff
        let mut buff = String::new();
        let stdin = io::stdin();

        // read line
        match stdin.read_line(&mut buff) {
            Err(_) => return None,
            Ok(size) => {
                buff.truncate(buff.len() - 1);
                match buff.is_empty() {
                    true => (),
                    false => self.list.push(buff),
                }
                return Some(size);
            }
        };
    }
}

impl TodoApp {
    fn print_list(&self) {
        println!("Returns: {:?}", self.list);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let mut todo_app: TodoApp = TodoApp { list: vec![] };

    loop {
        // read input
        match todo_app.read_line() {
            None => break,
            Some(_) => (),
        };
        // print out the list
        todo_app.print_list();
    }
}
