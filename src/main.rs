
mod app;

fn main() {
    let app = app::create_app();
    let matches = app.get_matches();
    println!("Matches: {:#?}",&matches.subcommand());
}
