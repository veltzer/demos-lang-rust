"""
os level dependencies for this project
"""

packages=[
    "rustc", # this provides "rustdoc" as well as "rustc"
    "cargo", # manages projects in rust
    "rust-src", # for debugging support, browsing source code and vscode auto complete
    # the two packages below do not exist on the CI/CD machines at github
    "clang",
    "clang-18",
    "lld",
    "lld-18",
    "cargo-doc",
    "cargo-1.80-doc",
    # "cargo-binutils",
    # "librust-cargo-binutils-dev",
    # ruby stuff
    "ruby-bundler",
    "rbenv",
    # for debugging
    "rustfilt",
    # for spelling
    "aspell",
]
