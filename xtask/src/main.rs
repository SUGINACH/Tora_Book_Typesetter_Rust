use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        None | Some("--help") | Some("-h") => {
            println!("xtask — tora-typesetter development tasks");
            println!("Usage: cargo run -p xtask -- <command>");
            println!("Commands will be added by the compatibility and build tasks.");
            ExitCode::SUCCESS
        }
        Some(command) => {
            eprintln!("Unknown xtask command: {command}");
            eprintln!("Run `cargo run -p xtask -- --help` for help.");
            ExitCode::from(2)
        }
    }
}
