use crate::app::project::Project;
use std::io::{Error, Write};
use std::fs::OpenOptions;

pub fn export_project_as_csv(project: &Project, target_file: &str) -> Result<(),Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target_file)?;
    let mut content = String::new();
    content.push_str("Projekt;Start;Ende;Tätigkeit;Zeit in Minuten\n");
    let name = &project.name;
    for task in &project.tasks {
        let task_start = &*task.start.to_rfc3339();
        let task_end = match &task.end {
            None => {String::new()},
            Some(date) => {date.to_rfc3339()},
        };
        let comment = match &task.comment{
            None => {""},
            Some(comment) => {&*comment},
        };
        let time = match task.end {
            None => {String::new()},
            Some(end) => {format!("{}",end.signed_duration_since(task.start))},
        };
        let row = format!("{};{};{};{};{}\n",name,task_start,task_end,comment,time);
        content.push_str(&row);
    }

    file.write_all(content.as_bytes())?;
    Ok(())
}