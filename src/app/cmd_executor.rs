use std::fs::DirBuilder;
use std::path::Path;

use crate::app::{WORKING_DIR, ARG_START_PROJECT, ARG_STOP_PROJECT, ARG_COMMENT, CHECKOUT_ARG_PROJECT, CHECKOUT_ARG_CREATE};
use clap::ArgMatches;
use crate::app::project::{create_project_manager, ProjectManager, ProjErr};
use std::error::Error;

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
        let mut mng = match create_project_manager() {
            Ok(m) => { m },
            Err(e) => {panic!("Fehler beim Projektmanagement: {}", e.description())},
        };
        if args.is_present(ARG_START_PROJECT) {
            let comment = args.value_of(ARG_COMMENT);
            mng.start_work(comment)?;
            return Ok(());
        } else if args.is_present(ARG_STOP_PROJECT) {
            mng.stop_work()?;
            return Ok(());
        }
        Ok(())
    }

    pub fn execute_checkout_cmd(args: &ArgMatches) -> Result<(),ProjErr> {
        let mut mng = match create_project_manager() {
            Ok(mng) => {mng},
            Err(e) => {panic!("Fehler beim initialieseren des ProjektManagers, {:#?}",e)},
        };
        let project = args.value_of(CHECKOUT_ARG_PROJECT).unwrap(); //catched by clap
        let create_if_not_exist = args.is_present(CHECKOUT_ARG_CREATE);
        mng.switch_project(project,create_if_not_exist)?;
        Ok(())
    }
}

