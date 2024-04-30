# rahat3062_minigrep

`rahat3062_minigrep` is a light-weight and minimal implementation of the grep command-line application. It is written in Rust and can be used both as a standalone application and as a library for other Rust programs.

## Binary Crate

install via:

```bash
cargo install rahat3062_minigrep && \
alias minigrep = rahat3062_minigrep
```

The binary crate provides a command-line interface to the grep functionality. You can use it to search for a string in a file. Here's how to run it:

```bash
minigrep <query> <filename> -s
```

Replace <query> with the string you want to search for and <filename> with the path to the file you want to search in and use -s for sensitive search.

If needed you can also take input from the stdin via pipelining:

````bash
history | minigrep cargo
```

## Library Crate

The library crate provides a `run` function that you can use to perform a grep search programmatically. Here's an example:

```rust
use rahat3062_minigrep::{Config, run};

let config = Config {
    query: "my query".to_string(),
    filename: Some("my_file.txt".to_string()),
    sensitive: false,
};

let result = run(config);
````

In this example, run searches for "my query" in "my_file.txt" and returns the matching lines. The search is case-insensitive because sensitive is false.
