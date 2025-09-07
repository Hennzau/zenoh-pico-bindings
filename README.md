# Rust bindings of Zenoh-Pico

> [!WARNING]
> **Only MacOS is supported for now**.

# Building

```bash
just clone-pico
just make-pico
just bindgen
```

# Examples

Each example can be run with arguments to set endpoints and modes for the zenoh session. Use `--help` to see the available options.

```bash
cargo run --example z_info
cargo run --example z_scout
cargo run --example z_put
```
