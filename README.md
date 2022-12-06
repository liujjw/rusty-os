# setup
tested on fedora 36 and ubuntu 18.04. `bin/setup.sh` and `export PATH=$HOME/.cargo/bin:$PATH` for rust. `arm gnu toolchain` also be required for c builds

# build and run
use `cargo`, unless `Makefile` provided in directory

# nota bene
- `ttywrite` crate tested to build only with `rustc nightly 1.67` and `optin_builtin_traits` -> `auto_traits` in shim, not `1.3x` from `setup.sh` (issue with `structopt` and `clap`) 
- rpi4 code assumes booting into default "low peripheral mode", so physical base address does NOT start at `0x7e000000` but `0xfe000000`. peripheral offsets unchanged.
- `qemu.sh` scripts should use `raspi4` and `cortex-a72`

# credits
based on [cs3210](https://github.com/sslab-gatech/cs3210-rustos-public), [rustos](https://github.com/phil-opp/blog_os), and [rpi4os](https://github.com/isometimes/rpi4-osdev)