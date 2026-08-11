mod app;
mod cli;
mod document;
mod editor_window;
mod screen;
mod terminal;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("nei: {error}");
        std::process::exit(1);
    }
}
