mod csv;
mod serializable;

use std::path::PathBuf;

fn main() {
    let args = cli_args();

    if args.is_empty() {
        print_help();
        return;
    }

    let directory = args[0].clone();

    init_project(&directory);
}

fn print_help() {
    println!("Hello!: todo help")
}

/// Initializes the project.
fn init_project<'a>(directory: &'a str) {
    const SAMPLES_DIRECTORY: &'static str = "samples";
    let directory = {
        let mut directory: PathBuf = directory.into();
        directory.push(SAMPLES_DIRECTORY);
        directory
    };

    let dir = |s: &str| -> PathBuf {
        let mut d = directory.clone();
        d.push(s);
        d
    };

    let routes = vec![dir("loops"), dir("one-shots")];

    for route in routes {
        std::fs::create_dir_all(route).unwrap();
    }
}

/// Returns the CLI arguments.
fn cli_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();

    // Pop off program name
    args.remove(0);

    args
}
