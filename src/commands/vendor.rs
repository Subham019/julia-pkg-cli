/*
 *vendor (from vendor.rs)/
 *   = readable source export for inspection
 *   = never used by Julia for loading
 *
 *.julia (from bundle.rs)/
 *   = actual local depot
 *   = used by jlpkg/Julia
 */
pub fn vendor(args: &[String]) {
    let deps_only = args.iter().any(|arg| arg == "--deps");
    let show_all = args.iter().any(|arg| arg == "--all");

    // Reject unknown options.
    for arg in args {
        if arg.starts_with("--") && arg != "--deps" && arg != "--all" {
            eprintln!("Unknown vendor option: {}", arg);
            eprintln!("Usage:");
            eprintln!("  jlpkg vendor");
            eprintln!("  jlpkg vendor --deps");
            eprintln!("  jlpkg vendor --all");
            eprintln!("  jlpkg vendor <package> [package...]");
            std::process::exit(1);
        }
    }

    // --deps and --all describe different scopes.
    if deps_only && show_all {
        eprintln!("--deps and --all cannot be used together");
        std::process::exit(1);
    }

    // Package names are their own selection mode.
    let names: Vec<&String> = args
        .iter()
        .filter(|arg| !arg.starts_with("--"))
        .collect();

    if !names.is_empty() && (deps_only || show_all) {
        eprintln!("Package names cannot be combined with --deps or --all");
        std::process::exit(1);
    }

    crate::common::run_julia(
        r#"
using Pkg

deps = Pkg.dependencies()
project_root = dirname(Base.active_project())
vendor_dir = joinpath(project_root, "vendor")

deps_only = "--deps" in ARGS
show_all = "--all" in ARGS

requested = Set(
    arg for arg in ARGS
    if !startswith(arg, "--")
)

function is_stdlib(pkg)
    isnothing(pkg.source) && return false

    source = normpath(pkg.source)
    stdlib = normpath(Sys.STDLIB)

    startswith(source, stdlib)
end

packages = if !isempty(requested)

    # Explicit package selection.
    selected = [
        pkg for pkg in values(deps)
        if pkg.name in requested && pkg.source !== nothing
    ]

    # Fail cleanly if a requested package wasn't found.
    found = Set(pkg.name for pkg in selected)
    not_found = sort!(collect(setdiff(requested, found)))

    if !isempty(not_found)
        println(stderr,
            "Package(s) not found in dependency graph: ",
            join(not_found, ", ")
        )
        exit(1)
    end

    selected

elseif deps_only

    # Direct [deps] only, excluding stdlibs.
    [
        pkg for pkg in values(deps)
        if pkg.source !== nothing && pkg.is_direct_dep && !is_stdlib(pkg)
    ]

elseif show_all

    # Entire dependency graph, including stdlibs.
    [
        pkg for pkg in values(deps)
        if pkg.source !== nothing
    ]

else

    # Default:
    # entire transitive dependency graph,
    # excluding Julia stdlibs.
    [
        pkg for pkg in values(deps)
        if pkg.source !== nothing && !is_stdlib(pkg)
    ]

end

sort!(packages, by = pkg -> pkg.name)

if isdir(vendor_dir)
    rm(vendor_dir; recursive=true, force=true)
end

mkpath(vendor_dir)

for pkg in packages
    dest = joinpath(vendor_dir, pkg.name)
    println("Vendoring ", pkg.name)
    cp(pkg.source, dest)
end

println()
println(
    "Vendored ",
    length(packages),
    " package(s) into ",
    vendor_dir
)
"#,
        args,
    );
}