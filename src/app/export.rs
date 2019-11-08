use crate::app::project::Project;
use std::io::{Error, Write};
use std::fs::OpenOptions;
use crate::app::task::Task;
use chrono::{DateTime, Utc, Local, TimeZone};

pub fn export_project_as_csv(project: &Project, target_file: &str) -> Result<(),Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target_file)?;
    let mut content = String::new();
    content.push_str("Projekt;Start;Ende;Tätigkeit;Zeit in Minuten\n");
    let name = &project.name;
    let mut minutes_sum = 0;
    for task in &project.tasks {
        let (row,task_duration) = task_to_row(name, &task);
        minutes_sum += task_duration;
        content.push_str(&row);
    }
    let hour = minutes_sum as f64 / 60.0;
    content.push_str(&format!(";;;;; Summe in Stunden: {}", hour));
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn task_to_row(name: &str, task: &Task) -> (String,i64) {
    let task_start = to_locale_date_string(&task.start);
    let task_end = match task.end {
        None => { String::new() },
        Some(date) => { to_locale_date_string(&date) },
    };
    let comment = match &task.comment {
        None => { "" },
        Some(comment) => { &*comment },
    };
    let task_duration = match task.end {
        None => {0},
        Some(end) => {
            end.signed_duration_since(task.start).num_minutes()
        },
    };
    let row = format!("{};{};{};{};{}\n", name, task_start, task_end, comment, task_duration);
    (row, task_duration)
}

fn to_locale_date_string(date: &DateTime<Utc>) -> String {
    let local_date = date.with_timezone(&Local);
    local_date.format("%d-%m-%y %H:%M Uhr").to_string()
}