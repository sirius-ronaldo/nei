const NAME: &str = "NEI — Norton Editor Inspired";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("{NAME}");
    println!();
    println!("Usage:");
    println!("  nei [FILE]");
    println!("  nei --help");
    println!("  nei --version");
    println!();
    println!("The full-screen editor will be implemented incrementally from Sprint 01.");
}

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--help" | "-h") => print_help(),
        Some("--version" | "-V") => println!("nei {VERSION}"),
        Some(file) => {
            println!("{NAME} {VERSION}");
            println!("Requested file: {file}");
            println!("Editor implementation starts in Sprint 01.");
        }
        None => {
            println!("{NAME} {VERSION}");
            println!("Editor implementation starts in Sprint 01.");
        }
    }
}
