use std::io::Write;
use std::vec;

struct Stack {
    items: Vec<i32>,
}
impl Stack {
    fn peek(&self) -> Option<&i32> {
        self.items.get(0)
    }
    fn push(&mut self, number: i32) {
        self.items.push(number);
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn pop(&mut self) -> Option<i32>  {
        self.items.pop()
    }
    fn new() -> Self {
        Self { items: vec![] }
    }
    fn print(&self) {
        for item in self.items.iter() {
            println!("{}", item);
        }
    }
}
fn main() {
    let mut stack = Stack::new();
    let mut s = String::new();
    loop {
        print!(">> ");
        std::io::stdout().flush().expect("IDK. Something wrong with stdout");
        std::io::stdin()
            .read_line(&mut s)
            .expect("Could not read from stdin");
        let s2 = s.trim().to_string();

        if s2.bytes().all(|x| x.is_ascii_digit()) {
            let number: i32 = s2.parse().expect("Should not happen");
            stack.push(number);
            s.clear();
            continue;
        }
        match s2.as_str() {
            "print" => {
                stack.print();
            }
            "help" => {
                println!("Help:");
                println!("quit|q|Q|Quit|QUIT -> All Exits the program");
                println!("dup -> Duplicates the first element on the stack");
                println!("drop -> Deletes the first element on the stack");
                println!("help -> Print this help menu.");
                println!("greater -> Compare two top elements on the stack. Return wheter a > b");
                println!("equal -> Compare two top elements on the stack. Return wheter a == b");
                println!("less -> Compare two top elements on the stack. Return wheter a < b");
                println!("len -> Get how many items in the stack");

            }
            "quit" | "q" | "Q" | "Quit" | "QUIT" => {
                println!("Goodbye !");
                break;
            }
            "dup" => {
                if let Some(value) = stack.peek() {
                    stack.push(*value);
                } else {
                    println!("No items in the stack");
                }
            }
            "drop" => {
                if stack.len() > 0 {
                    stack.pop();
                } else {
                    println!("No items in the stack");
                }
            }
            "greater" => {
                if stack.len() < 2 {
                    println!("Not enough items in the stack to perform operation.",);
                    s.clear();
                    continue;
                }

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a > b) as i32);
            }
            "less" => {
                if stack.len() < 2 {
                    println!("Not enough items in the stack to perform operation.",);
                    s.clear();
                    continue;
                }

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a < b) as i32);
            } 
            "equal" => {
                if stack.len() < 2 {
                    println!("Not enough items in the stack to perform operation.",);
                    s.clear();
                    continue;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                stack.push((a == b) as i32);
            }

            "len" => { println!("stack len => {}", stack.len());}
            "+" | "-" | "/" | "*" | "%" => {
                if stack.len() < 2 {
                    println!("Not enough items in the stack to perform operation.",);
                    s.clear();
                    continue;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(match s2.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    "%" => a % b,
                    _ => panic!("Wtf"),
                });
            }
            _ => {
                println!("Unrecognized option \"{}\"", s2);
            }
        }

        s.clear();
    }
}
