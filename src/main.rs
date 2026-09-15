mod commands;
mod uuid;
mod common;
mod help;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help::print_help();
        return;
    }

    let command = &args[1];
    let rest = &args[2..];

    // run and repl forward their arguments to Julia, so only a leading flag counts.
    let command_help = match command.as_str() {
        "run" | "r" | "repl" => {
            matches!(rest.first().map(String::as_str), Some("-h") | Some("--help"))
        }
        _ => rest.iter().any(|arg| arg == "-h" || arg == "--help"),
    };

    if command_help && help::print_command_help(command) {
        return;
    }

    match command.as_str() {
        "help" | "-h" | "--help" => help::print_help(),

        "new"             => commands::new::new(&args[2..]),
        "init"   | "i"    => commands::init::init(&args[2..]),
        "add"    | "a"    => commands::add::add(&args[2..]),
        "remove" | "rm"   => commands::remove::remove(&args[2..]),
        "update" | "up"   => commands::update::update(&args[2..]),
        "fetch"  | "sync" => commands::fetch::fetch(),
        "build"  | "b"    => commands::build::build(),
        "test"   | "t"    => commands::test::test(&args[2..]),
        "status" | "st"   => commands::status::status(),
        "tree"            => commands::tree::tree(&args[2..]),
        "why"             => commands::tree::why(&args[2..]),
        "run"    | "r"    => commands::run::run(&args[2..]),
        "repl"            => commands::repl::repl(&args[2..]),
        "vendor"          => commands::vendor::vendor(&args[2..]),
        "bundle"          => commands::bundle::bundle(),
        "check"           => commands::check::check(),
        "clean"           => commands::clean::clean(&args[2..]),
        "compat"          => commands::update::compat(),

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!();
            help::print_help();
            std::process::exit(1);
        }
    }
}
