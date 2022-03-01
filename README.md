# jump -- Fuzzy match directory names and jump

## Build and install

Assuming you have the Rust toolchain installed, you can build the
project with `cargo build --release`.

Then copy the binary at `./target/release/jump` to somewhere in your
path.

## Usage

Before first-time usage, use `jump --init` to initialize the database.
Then `cd` to some directories to start populating the database.

To integrate `jump` with your shell, add the following command to your
`~/.bashrc` file or similar:

``` bash
eval "$(jump --shell $(basename $SHELL))"
```

(See below for currently supported shells.)

Then use `j whatever` to fuzzy-jump to previously-visited directories.
Note that your query can have an arbitrary number of arguments in it.

## Supported shells

- [ ] bash
- [x] zsh

## Similar projects

This project is inspired by the following project, and borrows some
code for the shell integration: https://github.com/gsamokovarov/jump

This current project is my way of reinventing the wheel. :)
