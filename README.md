# Rust bindings of Zenoh-Pico

Only MacOS is supported for now.

# Building

```bash
just clone-pico
just make-pico
just bindgen
```

# Error: System error 22 + Bad access

```bash
just t # will run examples/z_info.rs
```

Sometimes you may get `[2025-09-04T19:40:42Z ERROR ::_z_report_system_error] System error: 22`

Sometimes you may also get a `Bad access` error (accessible from `lldb` when running `just lldb` then `run`).

Strangely, running `examples/z_info_working.rs` works just fine (with `just w`).)
