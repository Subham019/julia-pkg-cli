pub fn add(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: ");
        eprintln!("  jlpkg add <package> [package...]");
        eprintln!("  jlpkg add --from-file <file>");
        std::process::exit(1);
    }
    let packages = crate::common::resolve_packages(args);
    crate::common::run_pkg_command("add", &packages);
}