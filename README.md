# defmt-stdout

Forwards defmt frames to the standard output to make it usable on Linux desktops.

Supported targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu`
- `aarch64-unknown-linux-musl`

## Requirements

`.cargo/config.toml`

```toml
[build]
target = "host-tuple"  # substituted automatically, no need to modify

[target.'cfg(target_os="linux")']
rustflags = ["-C", "relocation-model=static", "-C", "link-arg=-Tdefmt.x"]
linker = "gcc"
runner = "./runner.sh"

[target.'cfg(target_arch="x86_64")']
rustflags = [
  "-C",
  "link-arg=-T/usr/lib/x86_64-linux-gnu/ldscripts/elf_x86_64.x",
]

[target.'cfg(target_arch="aarch64")']
rustflags = [
  "-C",
  "link-arg=-T/usr/lib/aarch64-linux-gnu/ldscripts/aarch64linux.x",
]

[env]
DEFMT_LOG = "trace"
```

An explicit `[build] target = ...` is required even when it matches the host. It triggers cargo to compile build scripts under the host target (separate from your binary target) so the `-Tdefmt.x` link arg is not applied to build-script linking, which would otherwise fail.

`runner.sh`

```sh
#!/bin/sh
$@ | defmt-print -e $1
```

```sh
chmod +x runner.sh
```

This way `cargo run` will be automatically piped into `defmt-print`

`main.rs`

```rust,ignore
use defmt_stdout as _;
```

## References

See the discussion [here](https://github.com/knurling-rs/defmt/issues/463) and [here](https://github.com/knurling-rs/defmt/issues/730) and the [defmt-serial](https://github.com/gauteh/defmt-serial/tree/main/example-std) project
