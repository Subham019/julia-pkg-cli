pub fn check() {
    let project_root = match crate::common::find_project_root() {
        Some(root) => root,
        None => {
            eprintln!("No Project.toml found in this directory or any parent directory.");
            std::process::exit(1);
        }
    };

    let local_depot = project_root.join(".julia");
    let mut command = std::process::Command::new("julia");
    command.arg(format!("--project={}", project_root.display()));

    if local_depot.is_dir() {
        eprintln!("Using local depot: {}", local_depot.display());
    } else {
        eprintln!("Warning: no local depot at .julia -- checking against the user depot instead");
    }

    crate::common::configure_local_depot(&mut command, &project_root);
    command.args([
        "-e",
        r#"
using Pkg, TOML
Pkg.offline(true)
Pkg.instantiate()

proj = TOML.parsefile(Base.active_project())

for name in sort!(collect(keys(get(proj, "deps", Dict()))))
    print("  loading $name ... ")
    @eval using $(Symbol(name))
    println("ok")
end

println("offline check passed")
"#,
    ]);

    let status = match command.status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Failed to execute Julia: {}", error);
            std::process::exit(1);
        }
    };

    if !status.success() {
        eprintln!("Offline check failed");
        std::process::exit(1);
    }
}