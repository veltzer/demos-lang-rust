<%!
    import pydmt.helpers.git
%>${"##"} Number of examples

Currently there are ${pydmt.helpers.git.count_files("src/**/*.rs")} examples in this repo.
