use std::io;

const NAME: &str = "NEI - Norton Editor Inspired";
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--help" | "-h") => {
            print_help();
            Ok(())
        }
        Some("--version" | "-V") => {
            println!("nei {VERSION}");
            Ok(())
        }
        Some(file) => crate::app::run(Some(file)),
        None => crate::app::run(None),
    }
}

fn print_help() {
    println!("{NAME}");
    println!();
    println!("Usage:");
    println!("  nei [FILE]");
    println!("  nei --help");
    println!("  nei --version");
    println!();
    println!("The full-screen editor starts with the terminal shell in Sprint 01.");
}
