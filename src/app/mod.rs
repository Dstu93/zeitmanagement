
pub mod work;
pub mod task;
pub mod project;

use clap::{App, SubCommand, Arg};

pub const PROJECT_DATABASE: &str = "projects";
pub const WORK_DAY_DB: &str = "work_day";
pub const WORKING_DIR: &str = ".tm/";

const VERSION: &str = "Version: 0.1";
const AUTHOR: &str = "Author: <dst>";

//subcommand names
const PROJECT_SUB_CMD: &str = "project";
const CHECKOUT_SUB_CMD: &str = "checkout";

//Argument names
const ARG_START_PROJECT: &str = "start";
const ARG_STOP_PROJECT: &str = "stop";

pub fn create_app<'a, 'b>() -> App<'a,'b> {
    let mut app = App::new("tm")
        .version(VERSION)
        .author(AUTHOR)
        .about("CLI zum protokolieren von Arbeitszeiten für Projekte und z.B. Homeoffice");

    let project_sub_cmd = create_project_cmd();
    let checkout_sub_cmd = create_checkout_cmd();

    //add sub cmds
    app = app.subcommand(project_sub_cmd)
        .subcommand(checkout_sub_cmd);

    app
}

fn create_project_cmd<'a, 'b>() -> App<'a,'b> {
    SubCommand::with_name(PROJECT_SUB_CMD)
        .version(VERSION)
        .author(AUTHOR)
        .about("Kommando zum verwalten von projekten")
        .arg(Arg::with_name(ARG_START_PROJECT)
            .takes_value(false)
            .conflicts_with_all(&[ARG_STOP_PROJECT])
            .help("startet einen neuen Task für das derzeitig aktive Projekt"))
        .arg(Arg::with_name(ARG_STOP_PROJECT)
            .takes_value(false)
            .conflicts_with_all(&[ARG_START_PROJECT])
            .help("stop die arbeit an derzeitigem Projekt und setzt die aktuelle Zeit als Ende am derztigen Task"))
}

fn create_checkout_cmd<'b, 'a>() -> App<'a,'b> {
    SubCommand::with_name(CHECKOUT_SUB_CMD)
        .version(VERSION)
        .author(AUTHOR)
        .about("Kommando zum wechseln zwischen den Projekten")
        .arg(Arg::with_name("create")
            .short("c")
            .long("create")
            .help("erstellt Projekt wenn es nicht existiert"))
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .help("Name des Projekts auf das gewechselt wird.")
            .value_name("PROJECT NAME")
            .required(true))
}

