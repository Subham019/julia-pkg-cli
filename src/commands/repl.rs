pub fn repl(args: &[String]) {
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
        std::process::exit(status.code().unwrap_or(1));
    }
}