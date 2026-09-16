# jlpkg

Command reference for `jlpkg`, a Cargo-style command-line interface for
Julia's package manager.

Requires `julia` on `PATH`. Run `jlpkg help` for a summary, or
`jlpkg <command> --help` for any single command.

## Rules

These apply across every command and are the things most easily got wrong.

- **Project discovery.** Commands search upward from the current directory
  for the nearest `Project.toml`, the way cargo finds `Cargo.toml`. The
  exceptions are `init`, which acts on the current directory, and
  `new <name>`, which creates `./<name>`.

- **Local depot.** If `./.julia` exists at the project root, every command
  uses it as the Julia depot instead of the user depot. `jlpkg bundle`
  creates it; deleting the directory reverts to the user depot.

- **Version bounds.** `add` records `[compat]` entries only when
  `Project.toml` has both a `name` and a `uuid`. A bare environment created
  by `Pkg.activate` has neither, so bounds are silently not written. Run
  `jlpkg init` to add those fields, then `jlpkg compat` to backfill bounds
  for dependencies already present.

- **`vendor` is not `bundle`.** `vendor/` holds source copies for reading
  and is never consulted when loading packages — Julia resolves its depot
  before it reads the project. `bundle` is what makes a project work
  offline.

- **Script paths.** `jlpkg run` resolves paths against the current
  directory, not the project root, because Julia inherits the working
  directory. Only `--project` is rewritten.

- **Bundles are platform-specific.** Binary artifacts and precompile caches
  are tied to the operating system, CPU and Julia version that built them.

- **`clean --global` leaves the project.** Without the flag `clean` touches
  only `./.julia`; with it, garbage is collected in the user depot, which
  affects every project on the machine.

## Commands

| Invocation                                        |                                                   |
| ------------------------------------------------- | ------------------------------------------------- |
| `jlpkg new <name>`                                | Create a new package in `./<name>`                |
| `jlpkg init [--name <name>] [--package]`          | Set up the current directory as a project         |
| `jlpkg compat`                                    | Record `[compat]` bounds for current dependencies |
| `jlpkg add <pkg>...`                              | Add dependencies and record their bounds          |
| `jlpkg add --from-file <file>`                    | Add packages listed one per line                  |
| `jlpkg remove <pkg>... [-y]`                      | Remove dependencies; confirms unless `-y`         |
| `jlpkg update [<pkg>...]`                         | Update within recorded bounds                     |
| `jlpkg status`                                    | List direct dependencies and versions             |
| `jlpkg fetch`                                     | Install the versions pinned in `Manifest.toml`    |
| `jlpkg build`                                     | Install, then precompile                          |
| `jlpkg test [<pkg>...]`                           | Run tests                                         |
| `jlpkg check`                                     | Verify the environment loads with no network      |
| `jlpkg clean [--global]`                          | Drop precompile caches and collect garbage        |
| `jlpkg run [julia-options] <script.jl> [args...]` | Run a script in the project environment           |
| `jlpkg repl [julia-options]`                      | Start a REPL in the project environment           |
| `jlpkg tree [--all]`                              | Print the dependency tree                         |
| `jlpkg why <pkg>...`                              | Explain why a dependency is present               |
| `jlpkg bundle`                                    | Build a self-contained depot in `./.julia`        |
| `jlpkg vendor [<pkg>...] \| --deps \| --all`      | Copy dependency sources into `./vendor`           |
| `jlpkg docs [-o <file>] [--force] [--stdout]`     | Write this reference into the current directory   |
| `jlpkg help`                                      | Show the command summary                          |
| `jlpkg version`                                   | Show the version (also `-V`, `--version`)         |

Aliases: `i` `a` `rm` `up` `st` `b` `t` `r` for `init` `add` `remove`
`update` `status` `build` `test` `run`, and `sync` for `fetch`.

`--from-file` is accepted by `add`, `remove`, `update` and `test`. Blank
lines and lines beginning with `#` are ignored.

## Exit codes

| | |
|---|---|
| `0` | Success |
| `1` | Failure — bad arguments, no `Project.toml` found, Julia reported an error, a refused overwrite, or a declined confirmation |
| *script's own* | `jlpkg run` propagates the exit code of the script it ran |

## Recipes

Reproduce an existing project:

```console
$ jlpkg fetch        # install exactly what Manifest.toml pins
$ jlpkg build        # ... and precompile
```

Start a project in a directory that already has code:

```console
$ jlpkg init
$ jlpkg add DataFrames CSV
```

Make a project work without a network:

```console
$ jlpkg bundle       # build ./.julia with packages, artifacts and registry
$ jlpkg check        # confirm it resolves and loads offline
```

Inspect why something is installed:

```console
$ jlpkg tree         # whole graph, standard libraries hidden
$ jlpkg why Parsers  # the chains that pull it in
```

## Scope

jlpkg does not reimplement dependency resolution. Every command shells out
to Julia's `Pkg`, so version resolution, `[compat]` semantics and manifest
format are Julia's, unchanged. What jlpkg adds is the command surface,
project discovery, the local depot, and `bundle`.
