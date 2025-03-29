# project manager

pm add: Create new project
pm list: Show all projects
pm show <name>: Display detailed project info
pm edit <name>: Modify project details
pm remove <name>: Delete project
pm open <name>: Open project directory
pm search <tag/technology>: Find projects

save data as JSON 

## planned structure
project_manager/
│
├── src/
│   ├── main.rs         # entry point
│   ├── cli.rs          # command-line interface handling
│   ├── project.rs      # project data structure
│   ├── storage.rs      # data persistence
│   └── commands/       # individual command implementations
│       ├── add.rs
│       ├── list.rs
│       ├── edit.rs
│       └── remove.rs
│
├── Cargo.toml          # project dependencies
└── README.md           # project documentation
