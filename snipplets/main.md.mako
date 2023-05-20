<%!
    import pydmt.helpers.git
%>${"##"} number of examples 

Currently there are ${pydmt.helpers.git.count_files("src/**/*.rs")} examples in this repo.
