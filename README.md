# jlpkg

A Cargo-style command-line interface for Julia's package manager.

Julia's `Pkg` already has the parts that matter — a manifest, a lockfile, caret
version bounds, a real resolver. What it doesn't have is a terse shell command,
so routine work means starting a REPL or writing `julia -e 'using Pkg; ...'` by
hand. `jlpkg` puts cargo's verbs in front of it:

```console
$ jlpkg add DataFrames CSV
$ jlpkg tree
$ jlpkg run --threads 6 fit.jl
```

It also fills the one real gap in `Pkg`: there is no equivalent of `cargo init`
for a directory that already contains files. `Pkg.generate` refuses a non-empty
directory, which is exactly the case you hit when adding a project file to
existing analysis code. `jlpkg init` handles it.

## Requirements

- **Julia** on `PATH`. Developed and tested against 1.13.
- **Rust** 1.85 or newer to build (the crate uses edition 2024).

No dependencies — the crate builds against `std` alone.

## Install

```console
$ git clone https://github.com/Subham019/julia-pkg-cli
$ cd julia-pkg-cli
$ cargo build --release
```

The binary is `target/release/jlpkg`. Copy it somewhere on your `PATH`.

## Quick start

Turn a directory that already has Julia code into a project:

```console
$ cd my-analysis
$ jlpkg init
Added project identity to .../my-analysis/Project.toml

$ jlpkg add DataFrames CSV MixedModels
$ jlpkg status
```

Reproduce it somewhere else:

```console
$ git clone <your-repo> && cd <your-repo>
$ jlpkg fetch      # install the exact versions in Manifest.toml
$ jlpkg build      # ... and precompile them
```

## Commands

Run `jlpkg help` for the full list, or `jlpkg <command> --help` for any one of
them.

| Command | |
|---|---|
| `new <name>` | Create a new package in `./<name>` |
| `init` | Set up the current directory as a project |
| `compat` | Record version bounds for the current dependencies |
| `add`, `a` | Add dependencies and record their bounds |
| `remove`, `rm` | Remove dependencies (asks for confirmation) |
| `update`, `up` | Update within the recorded bounds |
| `status`, `st` | List direct dependencies and versions |
| `fetch`, `sync` | Install the versions pinned in `Manifest.toml` |
| `build`, `b` | Install, then precompile |
| `test`, `t` | Run tests |
| `check` | Verify the environment loads with no network |
| `clean` | Drop precompile caches and collect garbage |
| `run`, `r` | Run a script in the project environment |
| `repl` | Start a REPL in the project environment |
| `tree` | Print the dependency tree |
| `why` | Explain why a dependency is present |
| `bundle` | Build a self-contained depot in `./.julia` |
| `vendor` | Copy dependency sources into `./vendor` |

Every command searches upward from the current directory for `Project.toml`,
the way cargo finds `Cargo.toml`, so they work from anywhere inside a project.

## Version bounds

`Pkg.add` writes `[compat]` entries automatically — but only when the active
project is a *package*, meaning `Project.toml` has both a `name` and a `uuid`.
A bare environment created by `Pkg.activate` has neither, so projects that grew
that way silently record no bounds at all, and `update` is then free to walk a
dependency across a major version.

`jlpkg init` adds those fields, which switches the behaviour on. For a project
that already has dependencies, `jlpkg compat` backfills bounds from the
versions currently resolved.

## `bundle` vs `vendor`

These look similar and are not interchangeable.

**`bundle`** builds a project-local depot at `./.julia` containing the packages,
binary artifacts, registry and precompile caches the project needs. Once it
exists, every `jlpkg` command uses it instead of `~/.julia`, and the project
builds and runs with no network. This is the real equivalent of
`cargo vendor` plus its source-replacement config. Verify it with `jlpkg check`.

```console
$ jlpkg bundle
$ jlpkg check
offline check passed
```

A bundle is platform-specific: binary artifacts and precompile caches are tied
to the operating system, CPU and Julia version that produced them.

**`vendor`** copies dependency *sources* into `./vendor` for reading. It is not
consulted when loading packages — Julia resolves its depot before it reads the
project, so no in-project directory can redirect that. Use it to inspect
sources, not to make a project self-contained.

## Platform support

Developed and tested on Windows. The only platform-specific code is the UUID
generator, which has arms for Windows (`BCryptGenRandom`), Linux (`getrandom`)
and macOS/BSD (`arc4random_buf`). The Linux and macOS arms compile but have not
been run — reports welcome.

## License

See [LICENSE](LICENSE).
