pub fn init(args: &[String]) {
    let package = args.iter().any(|arg| arg == "--package");

    let requested_name = if let Some(pos) = args.iter().position(|arg| arg == "--name") {
        match args.get(pos + 1) {
            Some(name) if !name.starts_with("--") => Some(name.as_str()),
            _ => {
                eprintln!("--name requires a package name");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let current_dir = std::env::current_dir()
        .expect("Failed to get current directory");

    /*
     * Read a top-level value from Project.toml.
     *
     * Stops when the first [table] begins, so a "name"
     * inside another TOML table is not mistaken for the project name.
     */
    fn read_top_level_value(content: &str, wanted_key: &str) -> Option<String> {
        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') {
                break;
            }

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') && key.trim() == wanted_key {
                return Some(value.trim().trim_matches('"').to_string());
            }
            
        }

        None
    }

    /*
     * Create a new Project.toml.
     */
    fn create_project_toml(project_name: &str, project_file: &std::path::Path) {
        let uuid = crate::uuid::uuid_v4();

        let project_toml = format!(
            "name = \"{}\"\nuuid = \"{}\"\nversion = \"0.1.0\"\n",
            project_name,
            uuid
        );

        std::fs::write(project_file, project_toml)
            .expect("Failed to write Project.toml");

        println!("Created {}", project_file.display());
    }

    /*
     * Upgrade an existing anonymous Project.toml.
     *
     * Existing [deps], [compat], etc. are preserved.
     * Only missing identity fields are added.
     */
    fn add_project_identity(project_file: &std::path::Path, project_name: &str) {
        let content = std::fs::read_to_string(project_file)
            .expect("Failed to read Project.toml");

        let has_name =
            read_top_level_value(&content, "name").is_some();

        let has_uuid =
            read_top_level_value(&content, "uuid").is_some();

        let has_version =
            read_top_level_value(&content, "version").is_some();

        let mut identity = String::new();

        // Add only fields that are missing.
        if !has_name {
            identity.push_str(
                &format!("name = \"{}\"\n", project_name)
            );
        }

        if !has_uuid {
            identity.push_str(
                &format!("uuid = \"{}\"\n", crate::uuid::uuid_v4())
            );
        }

        if !has_version {
            identity.push_str(
                "version = \"0.1.0\"\n"
            );
        }

        if !identity.is_empty() && !content.is_empty() {
            identity.push('\n');
        }

        identity.push_str(&content);

        std::fs::write(project_file, identity)
            .expect("Failed to update Project.toml");

        println!(
            "Added project identity to {}",
            project_file.display()
        );
    }

    /*
     * Create src/<project_name>.jl.
     */
    fn create_project_structure(project_name: &str, current_dir: &std::path::Path) {
        let src_dir = current_dir.join("src");

        std::fs::create_dir_all(&src_dir)
            .expect("Failed to create src directory");

        let module_file =
            src_dir.join(format!("{}.jl", project_name));

        let module_definition =
            format!("module {}\n\nend\n", project_name);

        if !module_file.exists() {
            std::fs::write(
                &module_file,
                module_definition,
            )
            .expect("Failed to create module file");

            println!("Created {}", module_file.display());
        } else {
            println!("Module file already exists");
        }
    }

    /**********************************************************/

    let folder_name = current_dir
        .file_name()
        .expect("Failed to get project name")
        .to_string_lossy();

    /*
     * FIX #2:
     *
     * If the directory is already a valid Julia name,
     * preserve it exactly.
     *
     * CMS_HGLM -> CMS_HGLM
     *
     * Only invalid names are normalized:
     *
     * julia-pkg-cli -> JuliaPkgCli
     */
    let default_name = if crate::common::is_valid_project_name(&folder_name) {
        folder_name.to_string()
    } else {
        crate::common::normalize_julia_name(&folder_name)
    };

    /*
     * Explicit --name has priority over the directory name.
     */
    let proposed_name = match requested_name {
        Some(name) => name.to_string(),
        None => default_name,
    };

    if !crate::common::is_valid_project_name(&proposed_name) {
        eprintln!("Invalid Julia project name: \"{}\"", proposed_name);
        std::process::exit(1);
    }

    let project_file =
        current_dir.join("Project.toml");

    /*
     * Determine the authoritative project name.
     */
    let project_name;

    if !project_file.exists() {
        /*
         * No Project.toml:
         *
         * create a new identified project.
         */
        create_project_toml(
            &proposed_name,
            &project_file,
        );

        project_name = proposed_name;
    } else {
        let content =
            std::fs::read_to_string(&project_file)
                .expect("Failed to read Project.toml");

        let existing_name =
            read_top_level_value(&content, "name");

        match existing_name {
            Some(name) => {
                /*
                 * Already an identified project.
                 */
                if !crate::common::is_valid_project_name(&name) {
                    eprintln!("Invalid Julia project name in Project.toml: \"{}\"", name);
                    std::process::exit(1);
                }

                if let Some(requested) = requested_name && requested != name {
                    eprintln!("Project.toml already exists with name \"{}\"; requested name was \"{}\"", name, requested);
                    std::process::exit(1);
                }
                
                println!("Project.toml already initialized");

                project_name = name;
            }

            None => {
                /*
                 * FIX #1:
                 *
                 * Project.toml exists, but has no identity.
                 * Preserve it and add name/uuid/version.
                 */
                add_project_identity(&project_file, &proposed_name);

                project_name = proposed_name;
            }
        }
    }

    if package {
        create_project_structure(&project_name, &current_dir);
    }
}

