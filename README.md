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

if you need specific features that do not come as default use `--features` flag like:

```bash
cargo add library-name@version --features feature1,feature2
```

But you can also manually add them in Cargo.toml:

```toml
[dependencies]
ndarray = "0.15"
rand = "0.9.1"
rand_distr = "0.5.1"
```

You can also add crates from git or from path, see more details in the [cargo add](https://doc.rust-lang.org/cargo/commands/cargo-add.html) documentation.

## Crates worth knowing of

Generally useful crates:
- `rand`/`rand_distr`: random number generators
- `ndarray`: multi-dimensional arrays
- `rayon`: simple parallelism
- `clap`: taking in CLI arguments etc.
- `serde`/`serde-json`: serialisation and deserialisation

Quality of life:
- `tracing`: more robust logging than `std::log`
- `tracing-opentelemetry`: feed your `tracing` logs into telemetry like `datadog`
- `metrics`: standard for counting things like memory usage, records processed per second etc.
- `thiserror`: macros for custom error types
- `itertools`: tons of useful methods to use with iterators like `.unique()` `.multizip()`, `.group_by()` etc.
- `chrono`: for actual datetime maths rather than bare-bones `std::time`
- `parking_lot`: faster mutexes for high performance concurrency
- `dashmap`: high performance concurrent hashmap if you have many threads accessing the data

Web stuff:
- `reqwest`: HTTP requests
- `axum`: web backend
- `tokio`: async stuff

Databases:
- `sqlx`: checks your query validity at compile time (postgres, sqlite, mysql)
- `diesel`: heavy duty ORM
- `connector_x`: ultra fast bridge between data coming from a database and `polars` or `ndarray`
- `object_store`: unified way of interfacing with `AWS S3`, `Azure Blob Storage` and `Google Cloud Storage`
- `snowflake-connector-rs`: not as good as `sqlx` but alright for snowflake, otherwise could use `arrow`
- `deltalake`: handling delta tables without a JVM

GUI/TUI:
- `ratatui`: TUI library
- `egui`: immeddiate mode GUI library
- `bevy`: ECS game engine, also great for 3D simulations
- `raylib-rs`: wrapper over `raylib`, good for drawing 2D shapes easily

Data Science/ML:
- `polars`: data frames, also look at `arrow` for `.parquet` stuff
- `plotters`: making plots
- `candle`: ML framework by huggingface, feels like `pytorch`
- `burn`: deep learning framework using `WGPU`
- `ort`: `ONNX` runtime to use pretrained models from Python
- `tch-rs`: bindings for `libtorch` to run `.pt` models
- `tinyvec`: tiny stack allocated vectors for speedy loops
- `memmap2`: `mmap` functionality for dealing with yuge files
- `faer`: linear algebra stuff at about `OpenBLAS` speeds

## Bin files - additional binaries

If you want a lot of runnable binaries you put them in `src/bin` and can call them:

```bash
cargo run --bin async-main
```

but this stops the ability to just use `cargo run` to run `main.rs` and instead need to specify the binary every time, hence I am not doing this here and extra examples that require modifying main are in the `other-examples` folder.
