# ruls

`ruls` (Rust + ls) is a simple `ls` clone written in Rust.

> Sounds like “rules” — because this ls clone *rules*.

## Features

* Lists non-hidden files in the current directory
* Sorted alphabetically
* Handles I/O errors gracefully
* Proper exit codes (0 on success, 1 on failure)
* Accepts paths (files and directories)
* Prints directory headers when multiple paths are provided

## Roadmap

| Version | Goal                                                           |
| ------- | -------------------------------------------------------------- |
| v0.1    | Basic listing (done)                                           |
| v0.2    | Paths as args, file vs dir handling, multi-path headers (done) |
| v0.3    | Add flags: `-a`, `-l`, `-r`, etc.                              |
| v0.4    | Colors, icons                                                  |
| v0.5+   | Recursive listing, tests, docs                                 |

## Build

```bash
cargo build --release
```

## Run

```bash
# no args: list current directory
./target/release/ruls

# files and directories as operands
./target/release/ruls Cargo.toml src
```

# TODO

- [ ] JSON/Table output

## Install (from binary) (from binary)

Download the latest release for your platform from
[GitHub Releases](https://github.com/XNinety9/ruls/releases).

Extract and move the binary somewhere in your `$PATH`.

## License

This project is licensed under the [MIT License](LICENSE).
