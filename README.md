# 🦀 Rust Syntax Revision Notes

Very simple repo to note down basic Rust syntax to look back on when learning.

## Setting up rust

On Linux and MacOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setting up a new Rust project

First create a directory and navigate into it:

```bash
mkdir project-name
cd project-name
```

Then initialise with:

```bash
cargo init
```

## Running and building

To build and run do:

```bash
cargo run
```

Or compile without running with:

```bash
cargo build
```

## Add crates to project

To import crates (Rust version of libraries) beyond `std` you need to include them in the `Cargo.toml` file, quickest way is to use cargo:

```bash
cargo add library-name@version
```

But you can also manually add them in Cargo.toml:

```toml
[dependencies]
ndarray = "0.16.1"
rand = "0.9.1"
rand_distr = "0.5.1"
```

You can also add crates from git or from path, see more details in the [cargo add](https://doc.rust-lang.org/cargo/commands/cargo-add.html) documentation.