use crate::app::cmd_executor::CmdExecutor;
use std::path::Path;
use crate::app::WORKING_DIR;
use crate::app::project::ProjErr;

mod app;

fn main() {
    let app = app::create_app();
    match app.get_matches().subcommand() {
        ("project",Some(args)) => {
            check_working_dir();
            if let Err(error) = CmdExecutor::execute_project_cmd(args) { print_error(error)};
        },
        ("checkout", Some(args)) => {
            check_working_dir();
            if let Err(error) = CmdExecutor::execute_checkout_cmd(args){print_error(error)};
        },
        ("init", Some(_args)) => {
            CmdExecutor::init_working_dir().unwrap();
        },
        _ => {println!("Geben Sie ein Kommando an. Siehe --help")}
    };
}

fn print_error(e: ProjErr) {
    match e {
        ProjErr::ProjNotFound => {
            println!("Projekt konnte nicht gefunden werden");
        },
        ProjErr::ProjAlreadyExists => {
            println!("Projekt konnte nicht erstellt werden weil es bereits existiert");
        },
        ProjErr::CommentAlreadyExists => {
            println!("Der Task besitzt bereits einen Kommentar")
        },
        ProjErr::NoTaskForProject => {
            println!("Es gibt keinen Task für dieses Projekt");
        },
        ProjErr::CouldNotWriteDB => {
            println!("Konnte Datenbank nicht beschreiben");
        },
    };
}

fn check_working_dir() {
    if !Path::new(WORKING_DIR).exists() {
        println!("Arbeitsverzeichnis nicht gefunden. Benutze init zum erstellen eines Arbeitsverzeichnis im aktuellen Ordner");
        std::process::exit(-1);
    }
}
