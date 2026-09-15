pub fn tree(args: &[String]) {
    for arg in args {
        if arg != "--all" {
            eprintln!("Unknown tree option: {}", arg);
            eprintln!("Usage: jlpkg tree [--all]");
            std::process::exit(1);
        }
    }
    crate::common::run_julia(
        r#"
using Pkg

deps = Pkg.dependencies()
show_all = "--all" in ARGS

function is_stdlib(pkg)
    isnothing(pkg.source) && return false
    source = normpath(pkg.source)
    stdlib = normpath(Sys.STDLIB)
    startswith(source, stdlib)
end

function show_dep(uuid, seen, prefix="", last=true)
    haskey(deps, uuid) || return
    pkg = deps[uuid]
    branch = last ? "└── " : "├── "
    version = isnothing(pkg.version) ? "" : " v$(pkg.version)"

    if uuid in seen
        println(prefix, branch, pkg.name, version, " (*)")
        return
    end

    println(prefix, branch, pkg.name, version)
    push!(seen, uuid)
    children = [
        child for child in values(pkg.dependencies) 
        if haskey(deps, child) && (show_all || !is_stdlib(deps[child]))
    ]
    sort!(children, by = u -> deps[u].name)
    child_prefix = prefix * (last ? "    " : "│   ")

    for (i, child) in enumerate(children)
        show_dep(child, seen, child_prefix, i == length(children))
    end
end

project = Pkg.project()
println(project.name === nothing ? "Project" : project.name)

roots = [
    uuid for (uuid, pkg) in deps 
    if pkg.is_direct_dep && (show_all || !is_stdlib(pkg))
]
sort!(roots, by = u -> deps[u].name)
seen = Set{Base.UUID}()

for (i, uuid) in enumerate(roots)
    show_dep(uuid, seen, "", i == length(roots))
end

"#,
        args,
    );
}

pub fn why(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: jlpkg why <package> [package...]");
        std::process::exit(1);
    }
    crate::common::run_pkg_command("why", args);
}