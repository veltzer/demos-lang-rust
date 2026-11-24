# TOFIX

Findings from a code scan on 2026-10-04.

## Medium

- `examples/macros/macro_basic/src/main.rs:23` - the example cannot compile: it defines a `#[proc_macro]` inside a binary crate (no `proc-macro = true` lib target), uses `syn`/`quote` without declaring them in `examples/macros/macro_basic/Cargo.toml`, and has a top-level `let` statement (rustc: "expected item, found keyword `let`"). It is also not a workspace member, so the build never notices. Split it into a proc-macro lib crate + a bin crate that uses it, declare the deps, and add it to `Cargo.toml` members.
- `Cargo.toml:5` - three crates live under the workspace root but are neither members nor in `workspace.exclude`: `examples/anyhow/context` (commented out at line 151), `examples/macros/macro_basic`, `examples/types/type_inference`. Cargo refuses to build any of them ("current package believes it's in a workspace when it's not"), so students cannot even `cargo run` them. Add the working ones to `members` and list deliberately broken ones in `exclude` (or move them under `errors/`).
- `examples/types/type_inference/src/main.rs:12` - intentionally fails (inserting `7.2` into a map inferred as `HashMap<String, i32>`), and its package name is `type_test` (`examples/types/type_inference/Cargo.toml:2`), not matching the directory. If it is an error demo it belongs in `errors/` with the other non-compiling examples; otherwise fix it.
- `examples/anyhow/context/Cargo.toml:7` - dependency versions are pinned without a comment saying why: `anyhow = "1.0.98"`, and likewise `rand = "0.10.3"` (`examples/lifetimes/lifetimes_using/Cargo.toml:7`, `exercises/guessing_game/Cargo.toml:9`, `exercises/producers_consumers/Cargo.toml:9`), `crossbeam-channel = "0.5.8"` (`exercises/producers_consumers/Cargo.toml:10`), `core_affinity = "0.8.0"`, `fork = "0.10.0"`, `inline-c = "0.1.7"`, `unicode-normalization = "0.1.19"`. Cargo.lock already pins the resolved versions; relax these to the loosest requirement that compiles (or `"*"`) or add a comment explaining each constraint.

## Low

- `rsconstruct.toml:64` - comment says the standalone demos are compiled "rustc on each src/*.rs in both release ... and debug ... Two instances, one per profile", but no such processor exists and `src/` holds only a placeholder `README.md`; the comment (and the Makefile/pydmt history in it) is stale. Trim it to what the `explicit.cargo` processor does.
- `scripts/publish.sh:3` - publishes the demo crate `map_simple@0.1.0` to crates.io; its manifest (`examples/hashmap/map_simple/Cargo.toml:5`) describes it as "A fast and secure library for parsing configuration files", which it is not (it is a small HashMap book-reviews demo binary). Drop the script or give the crate an honest description.
