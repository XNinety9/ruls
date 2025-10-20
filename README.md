# ruls

`ruls` (Rust + ls) is a simple `ls` clone written in Rust.

> Sounds like “rules” — because this ls clone *rules*.

## Features

* Lists non-hidden files in the current directory
* Sorted alphabetically
* Handles I/O errors gracefully
* Proper exit codes (0 on success, 1 on failure)

## Roadmap

| Version | Goal                              |
| ------- | --------------------------------- |
| v0.1    | Basic listing (done)              |
| v0.2    | Accept paths as arguments         |
| v0.3    | Add flags: `-a`, `-l`, `-r`, etc. |
| v0.4    | Colors, icons                     |
| v0.5+   | Recursive listing, tests, docs    |

## Build

```bash
cargo build --release
```

## Run

```bash
./target/release/ruls
```

## Install (from binary)

Download the latest release for your platform from
[GitHub Releases](https://github.com/XNinety9/ruls/releases).

Extract and move the binary somewhere in your `$PATH`.

## License

This project is licensed under the [MIT License](LICENSE).
