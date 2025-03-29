use std::fmt;

pub struct Project {
    pub name: String,
    pub path: String,
    //technologies: Vec<String>,
    //start_date: Date,
    pub status: ProjectStatus,
}

pub enum ProjectStatus {
    Planning,
    InProgress,
    Paused,
    Completed,
    Abandoned,
}

impl Project {
    pub fn serialize(&self) -> String {
        return format!(
            "{{\n\t\"name\": \"{}\",\n\t\"path\": \"{}\",\n\t\"status\": \"{}\",\n}}",
            self.name, self.path, self.status
        );
    }
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::InProgress => write!(f, "In Progress"),
            ProjectStatus::Completed => write!(f, "Completed"),
            ProjectStatus::Paused => write!(f, "Paused "),
            ProjectStatus::Abandoned => write!(f, "Abandoned"),
        }
    }
}
