packages=[
    "rustc", # this provides "rustdoc" as well as "rustc"
    "cargo", # manages projects in rust
    "rust-src", # for debugging support, browsing source code and vscode auto complete
    # the two packages below do not exist on the CI/CD machines at github
    # "cargo-binutils",
    # "librust-cargo-binutils-dev",
]
