use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

enum Command {
    Add { name: String, department: String },
    List { department: String },
}

fn main() {
    let mut people_by_department = HashMap::new();
    let re = Regex::new(r"^(?P<command>List|Add)(?:(?: (?P<name>\w+) to)? (?P<department>\w+))?$")
        .unwrap();

    loop {
        prompt_for_input();

        let input = read_input();
        match parse_command(&re, &input) {
            Ok(Command::Add { name, department }) => {
                people_by_department
                    .entry(department)
                    .or_insert_with(Vec::new)
                    .push(name);
            }
            Ok(Command::List { department }) => {
                if let Some(names) = people_by_department.get(&department) {
                    println!("People in {}:", department);
                    for name in names {
                        println!("- {}", name);
                    }
                } else {
                    println!("No one in {} department.", department);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn prompt_for_input() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(re: &Regex, input: &str) -> Result<Command, &'static str> {
    let caps = re.captures(input).ok_or("Failed to parse input")?;
    let command = caps.name("command").unwrap().as_str();
    match command {
        "Add" => {
            let name = caps
                .name("name")
                .map(|m| m.as_str().to_string())
                .ok_or("Missing name")?;
            let department = caps
                .name("department")
                .map(|m| m.as_str().to_string())
                .ok_or("Missing department")?;
            Ok(Command::Add { name, department })
        }
        "List" => {
            let department = caps
                .name("department")
                .map(|m| m.as_str().to_string())
                .ok_or("Missing department")?;
            Ok(Command::List { department })
        }
        _ => Err("Unknown command"),
    }
}

