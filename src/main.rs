mod project;

use project::{Project, ProjectStatus};
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help_menu();
        return;
    }

    match args[1].as_str() {
        "-h" => {
            help_menu();
        }

        "add" => {
            if args.len() == 3 {
                add_project(&args[2]);
            }
        }

        _ => print!("saa"),
    }
}

fn help_menu() {
    println!("Usage: pm [COMMAND]");
    println!("Commands:");
    println!("  add\tCreate new project");
    println!("  -h\tShow this help");
}

fn add_project(name: &String) {
    let p = Project {
        name: name.to_string(),
        path: "/home/xam/Projects/projectManager".to_string(),
        status: ProjectStatus::InProgress,
    };

    println!("{}", p.serialize());
}

fn set_name() {}

fn ask_for_path() {}
