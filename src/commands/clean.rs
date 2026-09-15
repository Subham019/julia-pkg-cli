pub fn clean(args: &[String]) {
    let global = args.iter().any(|arg| arg == "--global");

    for arg in args {
        if arg != "--global" {
            eprintln!("Unknown clean option: {}", arg);
            eprintln!("Usage: jlpkg clean [--global]");
            std::process::exit(1);
        }
    }

    let project_root = match crate::common::find_project_root() {
        Some(root) => root,
        None => {
            eprintln!("No Project.toml found in this directory or any parent directory.");
            std::process::exit(1);
        }
    };

    if !global {
        let local_depot = project_root.join(".julia");

        if !local_depot.is_dir() {
            println!("No local .julia depot found; nothing to clean.");
            return;
        }

        let compiled = local_depot.join("compiled");

        if compiled.is_dir() {
            if let Err(error) = std::fs::remove_dir_all(&compiled) {
                eprintln!("Failed to remove {}: {}", compiled.display(), error);
                eprintln!("Close any running Julia processes that may be using compiled package images.");
                std::process::exit(1);
            }

            println!("Removed {}", compiled.display());
        }

        println!("Garbage collecting local depot...");
        crate::common::run_julia("using Pkg; Pkg.gc()", &[]);

        return;
    }

    println!("Garbage collecting user depot...");

    let status = match std::process::Command::new("julia")
        .arg(format!("--project={}", project_root.display()))
        .args(["-e", "using Pkg; Pkg.gc()"])
        .env_remove("JULIA_DEPOT_PATH")
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