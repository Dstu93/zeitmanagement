use crate::app::task::Task;
use crate::app::{PROJECT_DATABASE, WORKING_DIR};

use serde::{Serialize,Deserialize};
use std::path::{PathBuf};
use std::fs::{File, OpenOptions};
use chrono::Utc;
use std::io::Error;

/// Schnittstelle zur Projektverwaltung
pub trait ProjectManager {
    /// wechselt zu einem neuen Projekt
    fn switch_project(&mut self, project: &str, create_if_not_exists: bool) -> Result<(), ProjErr>;
    fn create_project(&mut self, project: &str) -> Result<(),ProjErr>;
    fn current_project(&self) -> Option<String>;
    /// fuegt Kommentar an den derzeitigen Task an
    fn add_comment(&mut self, comment: &str, replace: bool) -> Result<(), ProjErr>;
    fn find(&self,project: &str) -> Result<&Project, ProjErr>;
    fn get_all(&self) -> Result<Vec<&Project>, ProjErr>;
    fn start_work(&mut self, comment: Option<&str>) -> Result<(),ProjErr>;
    fn stop_work(&mut self) -> Result<(),ProjErr>;
}

pub fn create_project_manager() -> Result<impl ProjectManager,Error> {
    Ok(ProjectManagerImpl::new()?)
}

/// Fehler der beim projektmanagement auftreten kann
#[derive(PartialOrd, PartialEq,Copy, Clone,Ord, Eq,Hash,Debug)]
pub enum ProjErr {
    ProjNotFound,
    ProjAlreadyExists,
    CommentAlreadyExists,
    NoTaskForProject,
    CouldNotWriteDB,
}

#[derive(PartialOrd, PartialEq,Clone,Debug,Hash,Serialize,Deserialize)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize,Deserialize,Debug)]
struct ProjectStorage {
    head: Option<String>,
    projects: Vec<Project>,
}

struct ProjectManagerImpl{
    storage: ProjectStorage,
    db_path: PathBuf,
}

impl ProjectManagerImpl {
    pub fn new() -> Result<ProjectManagerImpl,Error> {
        let mut db_path = PathBuf::from(WORKING_DIR);
            db_path.push(PROJECT_DATABASE);
        if !db_path.exists() {
            File::create(&db_path)?;
            let storage = ProjectStorage{head: None,projects: Vec::new()};
            let project_manager = ProjectManagerImpl{storage,db_path};
            let _ = project_manager.commit();
            return Ok(project_manager);
        }
        let db_file = File::open(&db_path).unwrap();
        let storage: ProjectStorage = serde_json::from_reader(db_file)
            .expect("Konnte Projekt Datenbank nicht lesen");
        Ok(ProjectManagerImpl{storage,db_path})
    }

    fn project_exists(&self, name: &str) -> bool {
        match self.find(name){
            Ok(_) => {true},
            Err(_) => {false},
        }
    }

    fn get_current_proj_mut(&mut self) -> Result<&mut Project,ProjErr> {
        let project_name = match &self.storage.head {
            Some(proj) => {proj},
            _ => panic!("head points to none")
        };
        for project in self.storage.projects.iter_mut() {
            if project.name.eq(project_name) {
                return Ok(project);
            }
        }
        Err(ProjErr::ProjNotFound)
    }

    fn commit(&self) -> Result<(),ProjErr>{
        let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&self.db_path).expect("Konnte DB nicht öffnen");
        match serde_json::to_writer_pretty(file,&self.storage){
            Ok(_) => {Ok(())},
            Err(e) => {
                println!("Fehler beim schreiben der Projektdatenbank. {:#?}",e);
                Err(ProjErr::CouldNotWriteDB)
            },
        }
    }
}

impl ProjectManager for ProjectManagerImpl {

    fn switch_project(&mut self, project: &str, create_if_not_exists: bool) -> Result<(), ProjErr> {
        if !self.project_exists(project) {
            if create_if_not_exists {
                self.create_project(project)?;
            }
            else { return Err(ProjErr::ProjNotFound) }
        }
        self.storage.head = Some(project.to_owned());
        self.commit()?;
        Ok(())
    }

    fn create_project(&mut self, project_name: &str) -> Result<(), ProjErr> {
        if self.project_exists(project_name) {
            return Err(ProjErr::ProjAlreadyExists);
        }

        let project = Project{ name: project_name.to_owned(), tasks: vec![] };
        self.storage.projects.push(project);
        self.commit();
        Ok(())
    }

    fn current_project(&self) -> Option<String> {
        self.storage.head.clone()
    }

    fn add_comment(&mut self, comment: &str, replace: bool) -> Result<(), ProjErr> {
        let project = self.get_current_proj_mut()?;
        let task = match project.tasks.last_mut() {
            Some(task) => {task},
            None => return Err(ProjErr::NoTaskForProject)
        };
        if task.comment.is_some() && !replace {
            return Err(ProjErr::CommentAlreadyExists);
        }
        task.comment = Some(comment.to_owned());
        self.commit()?;
        Ok(())
    }

    fn find(&self, project: &str) -> Result<&Project, ProjErr> {
        for stored_project in &self.storage.projects {
            if stored_project.name.eq(project){
                return Ok(stored_project);
            }
        }
        Err(ProjErr::ProjNotFound)
    }

    fn get_all(&self) -> Result<Vec<&Project>, ProjErr> {
        let projects = self.storage.projects.iter().collect();
        Ok(projects)
    }

    fn start_work(&mut self, comment: Option<&str>) -> Result<(), ProjErr> {
        let project = self.get_current_proj_mut()?;

        if let Some(task) = project.tasks.last_mut() {
            println!("Letzter Task wurde nich beendet. Beende letzten Task: {:#?}", &task);
            task.stop()
        };
        let comment = to_owned_string(comment);
        let new_task = Task{ start: Utc::now(), end: None, comment };
        project.tasks.push(new_task);
        self.commit()?;
        Ok(())
    }

    fn stop_work(&mut self) -> Result<(), ProjErr> {
        let project = self.get_current_proj_mut()?;
        let last_task = match project.tasks.last_mut() {
            Some(task) => {task},
            None => {return Err(ProjErr::NoTaskForProject)}
        };
        last_task.end = Some(Utc::now());
        self.commit()?;
        Ok(())
    }

}

fn to_owned_string(comment: Option<&str>) -> Option<String> {
    match comment {
        None => { None },
        Some(str) => { Some(str.to_string()) },
    }
}
