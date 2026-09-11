#!/usr/bin/env python

""" Build the cargo workspace.

Ignores the file arguments rsconstruct passes (the workspace root Cargo.toml
decides what is built) and runs `cargo build` on the whole workspace.

cargo owns its own `target/` directory and its own incremental cache, so we
do NOT let rsconstruct cache `target/`: hardlink-restoring it back read-only
makes the tree unusable by the next `cargo` invocation (it cannot open its
locks). Instead the processor's declared output is a tiny stamp file, which
is safe to cache and restore, and cargo is left to manage target/ itself. """

import pathlib
import subprocess
import sys
import time

STAMP = pathlib.Path("out/cargo.stamp")


def main():
    """ main entry point """
    code = subprocess.call(["cargo", "build", "--quiet"])
    if code:
        sys.exit(code)
    STAMP.parent.mkdir(parents=True, exist_ok=True)
    STAMP.write_text(f"cargo build succeeded at {time.time()}\n", encoding="utf-8")


if __name__ == "__main__":
    main()
