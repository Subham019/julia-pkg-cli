pub fn update(args: &[String]) {
    if args.is_empty() {
        crate::common::run_julia("using Pkg; Pkg.update()", &[]);
    } else {
        let packages = crate::common::resolve_packages(args);
        crate::common::run_pkg_command("update", &packages);
    }
}

pub fn compat() {
    crate::common::run_julia(
        "using Pkg; Pkg.compat(current=true)",
        &[],
    );
}