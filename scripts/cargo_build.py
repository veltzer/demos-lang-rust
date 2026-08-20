#!/usr/bin/env python

""" Build the cargo workspace. Ignores file arguments (rsconstruct passes the
tracked inputs); the workspace root Cargo.toml decides what is built. """

import subprocess
import sys


def main():
    """ main entry point """
    sys.exit(subprocess.call(["cargo", "build", "--quiet"]))


if __name__ == "__main__":
    main()
