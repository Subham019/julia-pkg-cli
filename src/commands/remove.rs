pub fn remove(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: ");
        eprintln!("  jlpkg remove <package> [package...]");
        eprintln!("  jlpkg remove --from-file <file>");
        std::process::exit(1);
    }
    
    let yes = args.iter().any(|arg| arg == "-y" || arg == "--yes");

    let package_args: Vec<String> = args
        .iter()
        .filter(|arg| *arg != "-y" && *arg != "--yes")
        .cloned()
        .collect();

    let packages = crate::common::resolve_packages(&package_args);
    
    if !yes {
        println!("Packages to remove:");
        for package in &packages { println!("  - {}", package); }
        print!("\nProceed? [y/N]: ");

        use std::io::Write;
        std::io::stdout().flush().expect("Failed to flush stdout");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).expect("Failed to read input");
        let confirmed = matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes");
        if !confirmed { println!("Cancelled."); return; }
    }
    crate::common::run_pkg_command("rm", &packages);
}