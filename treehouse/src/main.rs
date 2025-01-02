#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn whats_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("Bert", "Hello Bert, enjoy your time."),
        Visitor::new("Roger", "Hey Roger, thanks for coming by!"),
        Visitor::new("Daniel", "What a surprise!"),
    ];
    
    println!("What's your name?");
    let name = whats_your_name();
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);
        
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, you don't appear to be on the list.")
    }
}
    
