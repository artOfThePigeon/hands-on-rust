#![warn(clippy::all, clippy::pedantic)]

use std::io::{stdin, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Visitor {
    name: String,
    greeting: String,
    access_level: AccessLevel,
    visit_count: u32,
    last_visit: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
enum AccessLevel {
    Guest,
    Member,
    VIP,
    Admin,
}

impl Visitor {
    fn new(name: &str, greeting: &str, access_level: AccessLevel) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            access_level,
            visit_count: 0,
            last_visit: None,
        }
    }

    fn greet_visitor(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.visit_count += 1;

        // Print custom greeting based on visit frequency
        if let Some(last_visit) = self.last_visit {
            let time_since_last = current_time - last_visit;
            if time_since_last < 86400 {  // Within 24 hours
                println!("Welcome back so soon, {}! Glad to see you again!", self.name);
            } else if time_since_last < 604800 {  // Within a week
                println!("{}", self.greeting);
                println!("It's been {} days since your last visit.", time_since_last / 86400);
            }
        } else {
            println!("{}", self.greeting);
        }

        // Print status based on visit count
        match self.visit_count {
            1 => println!("This is your first visit - welcome aboard!"),
            5 => println!("Fifth visit! You're becoming a regular!"),
            10 => println!("Wow! Ten visits! You deserve a loyalty card!"),
            x if x % 50 == 0 => println!("🎉 Amazing! This is your {}th visit!", x),
            _ => (),
        }

        // Print access level perks
        match self.access_level {
            AccessLevel::Guest => println!("Feel free to look around the public areas."),
            AccessLevel::Member => println!("Members' lounge is on the second floor."),
            AccessLevel::VIP => println!("The VIP suite is ready for you, as always."),
            AccessLevel::Admin => println!("All areas are accessible. System status: NORMAL"),
        }

        self.last_visit = Some(current_time);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase()
}

fn attempt_login(tries: &mut HashMap<String, u32>, name: &str) -> bool {
    const MAX_TRIES: u32 = 3;
    let try_count = tries.entry(name.to_string()).or_insert(0);

    if *try_count >= MAX_TRIES {
        println!("⚠️  Access temporarily restricted due to too many attempts.");
        println!("Please contact security to reset your access.");
        return false;
    }

    let password = get_input("Please enter your access code: ");

    // Note: In a real system, you'd want proper password hashing
    if password == "1234" {
        *tries = HashMap::new(); // Reset all tries on successful login
        true
    } else {
        *try_count += 1;
        println!("❌ Invalid access code. {} attempts remaining.", MAX_TRIES - *try_count);
        false
    }
}

fn main() {
    let mut visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your exclusive time.", AccessLevel::VIP),
        Visitor::new("roger", "Hey Roger, thanks for being a loyal member!", AccessLevel::Member),
        Visitor::new("daniel", "Welcome back, Administrator Daniel.", AccessLevel::Admin),
        Visitor::new("sam", "Welcome, Sam!", AccessLevel::Guest),
    ];

    let mut login_attempts = HashMap::new();

    loop {
        println!("\n=== Visitor Management System ===");
        println!("1. Sign in as visitor");
        println!("2. View visitor statistics");
        println!("3. Exit");

        match get_input("Select an option (1-3): ").as_str() {
            "1" => {
                let name = get_input("What's your name? ");
                if let Some(visitor) = visitor_list
                    .iter_mut()
                    .find(|visitor| visitor.name == name)
                {
                    if visitor.access_level != AccessLevel::Guest {
                        if !attempt_login(&mut login_attempts, &name) {
                            continue;
                        }
                    }
                    visitor.greet_visitor();
                } else {
                    println!("Sorry, you don't appear to be on the list.");
                    println!("Would you like to register as a guest? (yes/no)");
                    if get_input("") == "yes" {
                        visitor_list = [
                            Visitor::new(&name, &format!("Welcome, {}!", name), AccessLevel::Guest),
                            visitor_list[0].clone(),
                            visitor_list[1].clone(),
                            visitor_list[2].clone(),
                        ];
                        println!("✅ Registration successful! Please sign in again.");
                    }
                }
            }
            "2" => {
                println!("\nVisitor Statistics:");
                for visitor in &visitor_list {
                    println!(
                        "{}: {} visits (Level: {:?})",
                        visitor.name, visitor.visit_count, visitor.access_level
                    );
                }
            }
            "3" => {
                println!("Thank you for using the Visitor Management System. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}