mod actor;
mod containers;
mod csv;
mod domain;

fn main() {
    let args = cli_args();

    let dir = "project";
    let mut domain = domain::Domain::new(dir.into());

    domain.save();
}

/// Returns the CLI arguments.
fn cli_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();

    // Pop off program name
    args.remove(0);

    args
}
