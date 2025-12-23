# defmt-stdout

Forwards defmt frames to the standard output to make it usable on Linux desktops.

## Requirements

`.cargo/config.toml`

```toml
[build]
target = "x86_64-unknown-linux-musl"

[target.x86_64-unknown-linux-musl]
rustflags = [
  "-C", "relocation-model=static",
  "-C", "link-arg=-T/usr/lib/x86_64-linux-gnu/ldscripts/elf_x86_64.x",
  "-C", "link-arg=-Tdefmt.x"
]
runner = "./runner.sh"

[env]
DEFMT_LOG = "trace"
```

`runner.sh`

```sh
#!/bin/sh
$@ | defmt-print -e $1
```

This way `cargo run` will be automatically piped into `defmt-print`


## References

See the discussion [here](https://github.com/knurling-rs/defmt/issues/463) and [here](https://github.com/knurling-rs/defmt/issues/730) and the [defmt-serial](https://github.com/gauteh/defmt-serial/tree/main/example-std) project