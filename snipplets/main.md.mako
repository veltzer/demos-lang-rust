<%!
    import pydmt.helpers.git
%>${"##"} Number of examples

Currently there are ${pydmt.helpers.git.count_files("examples/**/*.rs")} examples in this repo.
Currently there are ${pydmt.helpers.git.count_files("exercises/**/*.rs")} exercises in this repo.
