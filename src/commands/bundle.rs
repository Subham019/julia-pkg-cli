/*
 *vendor (from vendor.rs)/
 *   = readable source export for inspection
 *   = never used by Julia for loading
 *
 *.julia (from bundle.rs)/
 *   = actual local depot
 *   = used by jlpkg/Julia
 */
pub fn bundle() {
    let project_root = match crate::common::find_project_root() {
        Some(root) => root,
        None => {
            eprintln!("No Project.toml found in this directory or any parent directory.");
            std::process::exit(1);
        }
    };

    let depot = project_root.join(".julia");

    if let Err(error) = std::fs::create_dir_all(&depot) {
        eprintln!(
            "Failed to create bundle directory \"{}\": {}",
            depot.display(),
            error
        );
        std::process::exit(1);
    }

    println!("Bundling environment into {}", depot.display());

    crate::common::run_julia("using Pkg; Pkg.instantiate(); Pkg.precompile()", &[]);
    println!("Bundle ready at {}", depot.display());
}