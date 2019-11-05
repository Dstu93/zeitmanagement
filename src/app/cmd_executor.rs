use std::fs::DirBuilder;
use std::path::Path;

use crate::app::{WORKING_DIR, ARG_START_PROJECT, ARG_STOP_PROJECT, ARG_COMMENT, CHECKOUT_ARG_PROJECT, CHECKOUT_ARG_CREATE, EXPORT_PROJECT_ARG, EXPORT_FILE_ARG};
use clap::ArgMatches;
use crate::app::project::{create_project_manager, ProjectManager, ProjErr};
use crate::app::export::export_project_as_csv;

pub struct CmdExecutor;

impl CmdExecutor {

    pub fn init_working_dir() -> Result<(),std::io::Error>  {
        let working_dir_path = Path::new(WORKING_DIR);
        if working_dir_path.exists() {
            println!("Ordner ist bereits initialisiert.");
            return Ok(());
        } else {
            DirBuilder::new().create(working_dir_path)?;
        }
        Ok(())
    }

    pub fn execute_project_cmd(args: &ArgMatches) -> Result<(),ProjErr> {
        let mut mng = create_project_manager()?;
        match args.subcommand() {
            (ARG_START_PROJECT,Some(args)) => {
                println!("starte neuen Task");
                let comment = args.value_of(ARG_COMMENT);
                mng.start_work(comment)?;
                return Ok(());
            }
            (ARG_STOP_PROJECT,_) => {
                println!("Stoppe aktuellen Task");
                mng.stop_work()?;
                return Ok(());
            }
            _ => {println!("fehlende Commands für tm-project")}
        }
        Ok(())
    }

    pub fn execute_checkout_cmd(args: &ArgMatches) -> Result<(),ProjErr> {
        let mut mng = create_project_manager()?;

        let project = match args.value_of(CHECKOUT_ARG_PROJECT) {
            None => {return Err(ProjErr::ProjNotFound)},
            Some(project_name) => {project_name},
        };

        let create_if_not_exist = args.is_present(CHECKOUT_ARG_CREATE);
        mng.switch_project(project,create_if_not_exist)?;
        Ok(())
    }

    pub fn execute_export_cmd(args: &ArgMatches) -> Result<(),ExportError> {
        let project_name = match args.value_of(EXPORT_PROJECT_ARG) {
            None => {
                return Err(ExportError::MissingProjectArg);
            },
            Some(name) => {name},
        };

        let file_name = match args.value_of(EXPORT_FILE_ARG){
            None => {
                return Err(ExportError::MissingTargetFileArg);
            },
            Some(file_path) => {file_path},
        };

        let project_manager = create_project_manager()?;
        if let Ok(project) = project_manager.find(project_name) {
            export_project_as_csv(project,file_name)?;
        } else {
            return Err(ExportError::ProjectNotFound);
        }
        Ok(())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug,Hash)]
pub enum ExportError {
    IoError,
    MissingProjectArg,
    MissingTargetFileArg,
    ProjectNotFound,
}

impl From<std::io::Error> for ExportError {
    fn from(e: std::io::Error) -> Self {
        println!("IO Error beim Export: {:#?}",e);
        ExportError::IoError
    }
}

