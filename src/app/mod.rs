
pub mod work;
pub mod task;
pub mod project;
pub mod cmd_executor;
pub mod export;

use clap::{App, SubCommand, Arg};

pub const PROJECT_DATABASE: &str = "projects.db";
pub const WORK_DAY_DB: &str = "work_day";
pub const WORKING_DIR: &str = ".tm/";

const VERSION: &str = "Version: 0.1";
const AUTHOR: &str = "Author: <dst>";

//subcommand names
pub const PROJECT_SUB_CMD: &str = "project";
pub const CHECKOUT_SUB_CMD: &str = "checkout";
pub const INIT_SUB_CMD: &str = "init";

pub const EXPORT_SUB_CMD: &str = "export";
pub const EXPORT_FILE_ARG: &str = "csv_file";
pub const EXPORT_PROJECT_ARG: &str = "export_project";

//Argument names
pub const ARG_START_PROJECT: &str = "start";
pub const ARG_STOP_PROJECT: &str = "stop";
pub const ARG_COMMENT: &str = "comment";

pub const CHECKOUT_ARG_PROJECT: &str = "project";
pub const CHECKOUT_ARG_CREATE: &str = "create";

pub fn create_app<'a, 'b>() -> App<'a,'b> {
    let mut app = App::new("tm")
        .version(VERSION)
        .author(AUTHOR)
        .about("CLI zum protokolieren von Arbeitszeiten für Projekte und z.B. Homeoffice");

    let project_sub_cmd = create_project_cmd();
    let checkout_sub_cmd = create_checkout_cmd();
    let init_sub_cmd = create_init_cmd();
    let export_sub_cmd = create_export_cmd();

    //add sub cmds
    app = app.subcommands(vec![project_sub_cmd, checkout_sub_cmd, init_sub_cmd,export_sub_cmd]);

    app
}

fn create_project_cmd<'a, 'b>() -> App<'a,'b> {
    let stop_sub_cmd = SubCommand::with_name(ARG_STOP_PROJECT)
        .help("stop die arbeit an derzeitigem Projekt und setzt die aktuelle Zeit als Ende am derztigen Task");
    let start_sub_cmd = SubCommand::with_name(ARG_START_PROJECT)
        .help("startet einen neuen Task für das derzeitig aktive Projekt")
        .arg(Arg::with_name(ARG_COMMENT)
            .takes_value(true)
            .long("comment")
            .short("m")
            .help("Fügt ein Kommentar am Task an."));
    SubCommand::with_name(PROJECT_SUB_CMD)
        .version(VERSION)
        .author(AUTHOR)
        .about("Kommando zum verwalten von projekten")
        .subcommand(start_sub_cmd)
        .subcommand(stop_sub_cmd)
    //TODO ADD COMMENT
}

fn create_checkout_cmd<'b, 'a>() -> App<'a,'b> {
    SubCommand::with_name(CHECKOUT_SUB_CMD)
        .version(VERSION)
        .author(AUTHOR)
        .about("Kommando zum wechseln zwischen den Projekten")
        .arg(Arg::with_name(CHECKOUT_ARG_CREATE)
            .short("c")
            .long("create")
            .help("erstellt Projekt wenn es nicht existiert"))
        .arg(Arg::with_name(CHECKOUT_ARG_PROJECT)
            .short("p")
            .long("project")
            .help("Name des Projekts auf das gewechselt wird.")
            .value_name("PROJECT NAME")
            .required(true))
}

fn create_export_cmd<'a,'b>() -> App<'a,'b> {
    SubCommand::with_name(EXPORT_SUB_CMD)
        .author(AUTHOR)
        .version(VERSION)
        .about("Exportiert ein Projekt als CSV")
        .arg(Arg::with_name(EXPORT_FILE_ARG)
            .required(true)
            .long("file")
            .short("f")
            .value_name("DATEI")
            .takes_value(true))
        .arg(Arg::with_name(EXPORT_PROJECT_ARG)
            .required(true)
            .long("project")
            .short("p")
            .value_name("PROJEKT NAME")
            .takes_value(true))
}

fn create_init_cmd<'a,'b>() -> App<'a,'b> {
    SubCommand::with_name(INIT_SUB_CMD)
        .author(AUTHOR)
        .version(VERSION)
        .about("Erstellt im aktuellen Ordner das Arbeitsverzeichnis")
}

