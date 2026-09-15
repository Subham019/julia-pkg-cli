/// Print help for a single command. Returns false if the name is unknown,
/// so the caller can fall through to its usual unknown-command handling.
pub fn print_command_help(command: &str) -> bool {
    let text = match command {
        "new" => {
            "USAGE
    jlpkg new <name>

DESCRIPTION
    Create a new package in ./<name>, containing Project.toml and
    src/<name>.jl. The name must be a valid Julia identifier.
    To set up the directory you are already in, use `jlpkg init`.

OPTIONS
    -h, --help              Show this help"
        }

        "init" | "i" => {
            "USAGE
    jlpkg init [options]

DESCRIPTION
    Set up the current directory as a Julia project. Adds name, uuid and
    version to Project.toml, creating the file if absent and preserving any
    existing [deps] and [compat]. Unlike Pkg.generate, this works in a
    directory that already contains files.

OPTIONS
    --name <name>           Project name (default: the directory name)
    --package               Also create src/<Name>.jl
    -h, --help              Show this help"
        }

        "compat" => {
            "USAGE
    jlpkg compat

DESCRIPTION
    Record version bounds in [compat] for every direct dependency that
    lacks one, using the versions currently resolved in Manifest.toml.
    Bounds are caret ranges: \"1.8.2\" means >=1.8.2, <2.0.0.

OPTIONS
    -h, --help              Show this help"
        }

        "add" | "a" => {
            "USAGE
    jlpkg add <pkg>...
    jlpkg add --from-file <file>

DESCRIPTION
    Add packages as dependencies and record their version bounds in
    [compat]. Bounds are written only when Project.toml has a name and
    uuid; run `jlpkg init` first if it does not.

OPTIONS
    --from-file <file>      Read package names from a file, one per line.
                            Blank lines and # comments are ignored.
    -h, --help              Show this help"
        }

        "remove" | "rm" => {
            "USAGE
    jlpkg remove <pkg>...
    jlpkg remove --from-file <file>

DESCRIPTION
    Remove packages from the project. Their [compat] entries are removed
    along with them. Lists what will be removed and asks for confirmation
    unless -y is given.

OPTIONS
    --from-file <file>      Read package names from a file, one per line
    -y, --yes               Skip the confirmation prompt
    -h, --help              Show this help"
        }

        "update" | "up" => {
            "USAGE
    jlpkg update [<pkg>...]
    jlpkg update --from-file <file>

DESCRIPTION
    Update dependencies to the newest versions their [compat] bounds
    allow, and record the result in Manifest.toml. With no arguments,
    updates everything.

OPTIONS
    --from-file <file>      Read package names from a file, one per line
    -h, --help              Show this help"
        }

        "status" | "st" => {
            "USAGE
    jlpkg status

DESCRIPTION
    List the project's direct dependencies and their resolved versions.

OPTIONS
    -h, --help              Show this help"
        }

        "fetch" | "sync" => {
            "USAGE
    jlpkg fetch

DESCRIPTION
    Install the exact versions pinned in Manifest.toml without
    precompiling. If no manifest exists, resolve one from Project.toml.
    This is the command to run on a fresh clone.

OPTIONS
    -h, --help              Show this help"
        }

        "build" | "b" => {
            "USAGE
    jlpkg build

DESCRIPTION
    Install dependencies, then precompile them. Precompilation is the slow
    step; `jlpkg fetch` skips it.

OPTIONS
    -h, --help              Show this help"
        }

        "test" | "t" => {
            "USAGE
    jlpkg test [<pkg>...]

DESCRIPTION
    Run the project's tests, or the tests of the named packages.

OPTIONS
    --from-file <file>      Read package names from a file, one per line
    -h, --help              Show this help"
        }

        "check" => {
            "USAGE
    jlpkg check

DESCRIPTION
    Verify the environment is complete with no network access: resolves
    offline, then loads every direct dependency. Use it after `jlpkg
    bundle` to confirm the local depot is self-contained.

OPTIONS
    -h, --help              Show this help"
        }

        "clean" => {
            "USAGE
    jlpkg clean [--global]

DESCRIPTION
    Delete the local depot's precompile caches and collect unreachable
    packages. Does nothing if the project has no ./.julia depot.

OPTIONS
    --global                Collect garbage in the user depot (~/.julia)
                            instead. Affects every project on this machine.
    -h, --help              Show this help"
        }

        "run" | "r" => {
            "USAGE
    jlpkg run [julia-options] <script.jl> [args...]

DESCRIPTION
    Run a Julia script with the project activated. All arguments are
    passed to Julia unchanged, so Julia options work as usual and the
    script's own arguments arrive in ARGS. The script's exit code is
    propagated. Paths are resolved against the current directory, not
    the project root.

OPTIONS
    -h, --help              Show this help. Only honoured as the first
                            argument, since later ones belong to Julia."
        }

        "repl" => {
            "USAGE
    jlpkg repl [julia-options]

DESCRIPTION
    Start an interactive Julia REPL with the project activated. All
    arguments are passed to Julia unchanged.

OPTIONS
    -h, --help              Show this help. Only honoured as the first
                            argument, since later ones belong to Julia."
        }

        "tree" => {
            "USAGE
    jlpkg tree [--all]

DESCRIPTION
    Print the project's dependency tree. A subtree already shown
    elsewhere is marked (*) rather than repeated.

OPTIONS
    --all                   Include Julia standard libraries
    -h, --help              Show this help"
        }

        "why" => {
            "USAGE
    jlpkg why <pkg>...

DESCRIPTION
    Explain why a package is present, by listing the dependency chains
    that reach it. A direct dependency prints just its own name.

OPTIONS
    -h, --help              Show this help"
        }

        "vendor" => {
            "USAGE
    jlpkg vendor [<pkg>...]
    jlpkg vendor --deps
    jlpkg vendor --all

DESCRIPTION
    Copy dependency sources into ./vendor for inspection. With no
    arguments, copies the whole dependency graph excluding standard
    libraries.

    This directory is not used when loading packages -- Julia resolves the
    depot before it reads the project. For a self-contained environment,
    use `jlpkg bundle`.

OPTIONS
    --deps                  Direct dependencies only
    --all                   Whole graph, including standard libraries
    -h, --help              Show this help"
        }

        "bundle" => {
            "USAGE
    jlpkg bundle

DESCRIPTION
    Build a project-local Julia depot in ./.julia, containing the
    packages, artifacts, registry and precompile caches the project
    needs. Once it exists, every jlpkg command uses it instead of the
    user depot, and the project works with no network.

    The bundle is platform-specific: artifacts and precompile caches are
    tied to this operating system, CPU and Julia version.

OPTIONS
    -h, --help              Show this help"
        }

        _ => return false,
    };

    println!("{text}");
    true
}

pub fn print_help() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        r#"jlpkg {VERSION}
A Cargo-style command-line interface for Julia's package manager.

USAGE
    jlpkg <command> [arguments]

PROJECT
    new <name>              Create a new package in ./<name>
    init [options]          Set up the current directory as a project
    compat                  Record version bounds for the current dependencies

DEPENDENCIES
    add, a <pkg>...         Add dependencies and record their version bounds
    remove, rm <pkg>...     Remove dependencies
    update, up [<pkg>...]   Update dependencies within their recorded bounds
    status, st              List direct dependencies and their versions

ENVIRONMENT
    fetch, sync             Install the versions pinned in Manifest.toml / Project.toml (if Manifest not available)
    build, b                Install, then precompile
    test, t [<pkg>...]      Run tests
    check                   Verify the environment loads with no network
    clean [--global]        Drop precompile caches and collect garbage

RUNNING
    run, r <script> [args]  Run a script in the project environment
    repl [julia-options]    Start a REPL in the project environment

INSPECTION
    tree [--all]            Print the dependency tree
    why <pkg>...            Explain why a dependency is present

DISTRIBUTION
    bundle                  Build a project-local depot in ./.julia
    vendor [<pkg>...]       Copy dependency sources into ./vendor

OPTIONS
    init   --name <name>    Project name (default: the directory name)
           --package        Also create src/<Name>.jl
    add    --from-file <f>  Read package names from a file, one per line
    remove --from-file <f>  As above
           -y, --yes        Skip the confirmation prompt
    tree   --all            Include Julia standard libraries
    clean  --global         Collect garbage in the user depot (~/.julia)

VENDOR MODES
    jlpkg vendor                    Whole dependency graph, excluding standard libraries
    jlpkg vendor <pkg>              Only the named package
    jlpkg vendor <pkg1> <pkg2>...   Only the named packages
    jlpkg vendor --deps             Direct dependencies only
    jlpkg vendor --all              Whole dependency graph, including standard libraries

EXAMPLES
    jlpkg init --package            Make the current folder a package
    jlpkg add DataFrames CSV        Add two dependencies
    jlpkg run --threads 6 fit.jl    Run a script with 6 threads
    jlpkg bundle && jlpkg check     Build a depot, then verify it offline

Commands act on the nearest Project.toml, searching upward from the current
directory. If .julia exists at the project root, it is used as the local depot 
instead of the normal user depot.

Run `jlpkg help`, `jlpkg -h` or `jlpkg --help` to see help."#
    );
}
