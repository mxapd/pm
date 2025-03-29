use serde::

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

impl serialize for Project {}
