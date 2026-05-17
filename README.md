# defmt-stdout

Forwards defmt frames to the standard output to make it usable on Linux desktops.

Supports `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`.

## Requirements

`.cargo/config.toml`

```toml
# Pick the line that matches your host:
[build]
target = "x86_64-unknown-linux-gnu"   # or "aarch64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "gcc"
rustflags = [
  "-C", "relocation-model=static",
  "-C", "link-arg=-T/usr/lib/x86_64-linux-gnu/ldscripts/elf_x86_64.x",
  "-C", "link-arg=-Tdefmt.x",
]
runner = "./runner.sh"

[target.aarch64-unknown-linux-gnu]
linker = "gcc"
rustflags = [
  "-C", "relocation-model=static",
  "-C", "link-arg=-T/usr/lib/aarch64-linux-gnu/ldscripts/aarch64linux.x",
  "-C", "link-arg=-Tdefmt.x",
]
runner = "./runner.sh"

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
