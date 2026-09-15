pub fn test(args: &[String]) {
    if args.is_empty() {
        crate::common::run_julia("using Pkg; Pkg.test()", &[]);
    } else {
        let packages = crate::common::resolve_packages(args);
        crate::common::run_pkg_command("test", &packages);
    }
}