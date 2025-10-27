# Tauri Template

- Typescript and pnpm
- PostCSS
- Rust toolchain file and rustfmt
- VSCode tasks

## How to build

Development server
```sh
pnpm tauri dev
```

To build for distributing, run
```sh
pnpm tauri build
```

The Typescript bindings are generated from the Rust datatypes using `ts-rs`, so that Rust is the single source of truth.
To generate these manually, use
```sh
cargo test export_bindings
```