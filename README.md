# MINIGREP

A command line tool to search for contents in a file, written in Rust.

## Steps to play around with the tool:

1. Clone this repo.
2. Make sure rust is installed on your machine.
3. Run `cargo run -- <query> <file_path>` to run the tool. For example, `cargo run -- to poem.txt`.
4. Set IGNORE_CASE environment variable to do a case insensitive search. For example, `IGNORE_CASE=1 cargo run -- to poem.txt`
