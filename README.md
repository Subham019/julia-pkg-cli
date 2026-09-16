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

## How it compares

| jlpkg              | cargo                     | Julia `Pkg`                  |
| ------------------ | ------------------------- | ---------------------------- |
| `jlpkg new <name>` | `cargo new`               | `Pkg.generate("<name>")`     |
| `jlpkg init`       | `cargo init`              | — ¹                          |
| `jlpkg add X`      | `cargo add X`             | `Pkg.add("X")`               |
| `jlpkg remove X`   | `cargo remove X`          | `Pkg.rm("X")`                |
| `jlpkg update`     | `cargo update`            | `Pkg.update()`               |
| `jlpkg compat`     | *implicit in `cargo add`* | `Pkg.compat(current=true)` ² |
| `jlpkg status`     | —                         | `Pkg.status()`               |
| `jlpkg fetch`      | `cargo fetch`             | `Pkg.instantiate()`          |
| `jlpkg build`      | `cargo build`             | `Pkg.precompile()`           |
| `jlpkg test`       | `cargo test`              | `Pkg.test()`                 |
| `jlpkg run f.jl`   | `cargo run`               | `julia --project f.jl`       |
| `jlpkg repl`       | —                         | `julia --project`            |
| `jlpkg tree`       | `cargo tree`              | — ³                          |
| `jlpkg why X`      | `cargo tree -i X`         | `Pkg.why("X")`               |
| `jlpkg clean`      | `cargo clean`             | `Pkg.gc()`                   |
| `jlpkg bundle`     | `cargo vendor`            | — ⁴                          |
| `jlpkg check`      | `cargo build --offline`   | —                            |
| `jlpkg vendor`     | —                         | —                            |
| `jlpkg docs`       | —                         | —                            |

¹ `Pkg.generate` refuses a directory that already contains files, so there is
no way to add a project file to existing code. `jlpkg init` does it in place —
the gap that prompted this tool.

² `Pkg.add` writes `[compat]` bounds automatically, but only when
`Project.toml` has both a `name` and a `uuid`. A bare environment created by
`Pkg.activate` has neither, so such projects silently record no bounds at all
and `update` is free to walk a dependency across a major version. `jlpkg init`
adds the fields; `jlpkg compat` backfills bounds for what is already there.

³ `Pkg.status()` is flat. `jlpkg tree` renders the whole graph, marking a
subtree already shown with `(*)` and hiding standard libraries unless `--all`
is given.

⁴ See below.

## `bundle` and `vendor`

These look similar and are not interchangeable.

**`bundle`** builds a project-local depot at `./.julia` holding the packages,
binary artifacts, registry and precompile caches the project needs. Once it
exists, every `jlpkg` command uses it instead of `~/.julia`, and the project
builds and runs with no network. This is the real counterpart to `cargo vendor`
*plus* its source-replacement config — Julia has no per-project setting that
can redirect the depot, so the tool supplies it.

```console
$ jlpkg bundle
$ jlpkg check
offline check passed
```

A bundle is platform-specific: artifacts and precompile caches are tied to the
operating system, CPU and Julia version that produced them.

**`vendor`** copies dependency *sources* into `./vendor` for reading. It is
never consulted when loading packages, because Julia resolves its depot before
it reads the project. Use it to inspect source, not to make a project
self-contained.

## Reference

`jlpkg help` lists every command and `jlpkg <command> --help` documents one.
The full reference — project discovery, exit codes, and the rules that apply
across commands — is in [jlpkg.md](jlpkg.md), which `jlpkg docs` writes
into the current directory.

## Platform support

Developed and tested on Windows. The only platform-specific code is the UUID
generator, which has arms for Windows (`BCryptGenRandom`), Linux (`getrandom`)
and macOS/BSD (`arc4random_buf`). The Linux and macOS arms compile but have not
been run — reports welcome.

## License

MIT. See [LICENSE](LICENSE).
