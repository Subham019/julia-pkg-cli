pub fn new(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: jlpkg new <name>");
        std::process::exit(1);
    }

    let name = &args[0];

    if !crate::common::is_valid_project_name(name) {
        eprintln!("Invalid project name: {}", name);
        std::process::exit(1);
    }

    if std::path::Path::new(name).exists() {
        eprintln!("Destination already exists: {}", name);
        std::process::exit(1);
    }

    let status = match std::process::Command::new("julia")
        .args(["-e", "using Pkg; Pkg.generate(ARGS[1])", "--", name])
        .status()
    {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Failed to execute Julia: {}", error);
            std::process::exit(1);
        }
    };

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}