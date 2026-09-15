/*
 * Check whether a name is already a usable Julia identifier.
 */
pub fn is_valid_project_name(name: &str) -> bool {
    let mut chars = name.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/*
 * Normalize only names that are NOT already valid.
 *
 * julia-pkg-cli -> JuliaPkgCli
 * my package    -> MyPackage
 */
pub fn normalize_julia_name(name: &str) -> String {
    name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();

            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect()
}

pub fn find_project_root() -> Option<std::path::PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("Project.toml").exists() {return Some(dir);}
        if !dir.pop() {return None;}
    }
}

pub fn resolve_packages(args: &[String]) -> Vec<String> {
    if args.is_empty() {
        eprintln!("No packages specified");
        std::process::exit(1);
    }

    let file = if args[0] == "--from-file" {
        match args.get(1) {
            Some(file) => Some(file.as_str()),
            None => {
                eprintln!("--from-file requires a file path");
                std::process::exit(1);
            }
        }
    } else if let Some(file) = args[0].strip_prefix("--from-file=") {
        if file.is_empty() {
            eprintln!("--from-file requires a file path");
            std::process::exit(1);
        }
        Some(file)
    } else {None};

    if let Some(file) = file {
        let content = match std::fs::read_to_string(file) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Failed to read package file \"{}\": {}", file, error);
                std::process::exit(1);
            }
        };

        let packages: Vec<String> = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with('#'))
            .map(String::from)
            .collect();

        if packages.is_empty() {
            eprintln!("No packages found in {}", file);
            std::process::exit(1);
        }

        packages
    } else {args.to_vec()}
}

pub fn configure_local_depot(command: &mut std::process::Command, project_root: &std::path::Path) {
    let depot = project_root.join(".julia");

    if depot.is_dir() {
        let mut depot_path = depot.as_os_str().to_os_string();

        if cfg!(windows) {depot_path.push(";");} 
        else {depot_path.push(":");}

        command.env("JULIA_DEPOT_PATH", depot_path);
    }
}

pub fn run_julia(code: &str, args: &[String]) {
    /* root lookup + spawn + status */
    let project_root = match find_project_root() {
        Some(root) => root,
        None => {
            eprintln!("No Project.toml found in this directory or any parent directory.");
            std::process::exit(1);
        }
    };

    let mut command = std::process::Command::new("julia");

    command
        .arg(format!("--project={}", project_root.display()))
        .args(["-e", code, "--"])
        .args(args);

    configure_local_depot(&mut command, &project_root);
    let status = match command.status()
    {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Failed to execute Julia: {}", error);
            std::process::exit(1);
        }
    };

    if !status.success() {
        eprintln!("Julia command failed");
        std::process::exit(1);
    }
}

pub fn run_pkg_command(julia_function: &str, packages: &[String]) {
    let julia_code = format!("using Pkg; Pkg.{}(ARGS)", julia_function);
    run_julia(&julia_code, packages);
}
