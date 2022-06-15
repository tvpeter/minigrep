# minigrep

A simple CLI program built with `Rust` that searches a query in a file. It is built to replicate `grep` tool.

## Running the application

To run the cli program, use cargo and supply the search term `query` and the filename to search in this instance `poem.txt` to search .i.e

`cargo run to poem.txt`

where the `to` is the word been searched for

`poem.txt` is the file been searched

## Environment Variable

You can make your search case insensitive by setting `CASE_INSENSITIVE` to true or false.

`export CASE_INSENSITIVE=true`

 or you can unset it using `unset CASE_INSENSITIVE`

