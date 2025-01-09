mod gui;
mod logic;
use gui::app::App;
use relm4::RelmApp;
use std::env;
#[macro_use]
mod macros;

fn main() {
    let app = RelmApp::new("relm4.sudoku");

    println!(
        "Current working directory: {}",
        env::current_dir().unwrap().display()
    );

    if let Err(err) = relm4::set_global_css_from_file("src/style.css") {
        eprintln!("Failed to load CSS file: {}", err);
    }

    app.run::<App>(0);
}
