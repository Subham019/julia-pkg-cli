pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: ");
        eprintln!("  jlpkg run <script.jl> [args...]");
        eprintln!("  jlpkg run <julia-options> <script.jl> [args...]");
        std::process::exit(1);
    }

    let project_root = match crate::common::find_project_root() {
        Some(root) => root,
        None => {
            eprintln!("No Project.toml found in this directory or any parent directory.");
            std::process::exit(1);
        }
    };

    let mut command = std::process::Command::new("julia");

    command
        .arg(format!("--project={}", project_root.display()))
        .args(args);

    crate::common::configure_local_depot(&mut command, &project_root);

    let status = match command.status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Failed to execute Julia: {}", error);
            std::process::exit(1);
        }
    };

    if !status.success() {
        std::process::exit(
            status.code().unwrap_or(1)
        );
    }
}