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
    println!("Terminal text editor inspired by Norton Editor 1.3.");
    println!();
    println!("Usage:");
    println!("  nei [FILE]");
    println!("  nei --help");
    println!("  nei --version");
    println!();
    println!("Options:");
    println!("  -h, --help       Show this help");
    println!("  -V, --version    Show the version");
    println!();
    println!("If FILE does not exist, NEI opens an empty document with that name.");
    println!("Press F1 inside the editor for the keyboard command reference.");
}
