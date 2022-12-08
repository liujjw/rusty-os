# setup
tested on fedora 36 and ubuntu 18.04. `bin/setup.sh` and `export PATH=$HOME/.cargo/bin:$PATH` for rust. `arm gnu toolchain` also be required for c builds

# build 
use `cargo`, unless `Makefile` provided in directory. 

# run 
end result of build step should be a binary; rename to `kernel8.img` and paste into boot partition of rpi4 storage (using default `.py` install script will overwrite other files). rpi4 storage should already be initialized with [rpi4 imager](https://www.raspberrypi.com/software/) (we only modify `kernel8.img`).

## run configs
use default `configs.txt` in `boot` partition from raspbian 32-bit os install from the imager EXCEPT:
- force 64-bit mode by deleting other `kernel`s except `kernel8.img` OR set `arm_64bit=1`
- (default) 64-bit low peripheral mode (important for peripheral register base address)
- set `core_freq_min=500` for mini UART baud rate calculation to work properly

## uart
`sudo gpasswd --add <your-username> dialout` or use `sudo` before `sudo screen /dev/ttyUSB0 115200` to access mini UART

# nota bene
- `ttywrite` crate tested to build only with `rustc nightly 1.67` and `optin_builtin_traits` -> `auto_traits` in shim, not `1.3x` from `setup.sh` (issue with `structopt` and `clap`) 
- rpi4 code assumes booting into default "low peripheral mode", so physical base address does NOT start at `0x7e000000` but `0xfe000000`. peripheral offsets unchanged.
- `qemu.sh` scripts should use `raspi4` and `cortex-a72`

# credits
based on [cs3210](https://github.com/sslab-gatech/cs3210-rustos-public), [rustos](https://github.com/phil-opp/blog_os), and [rpi4os](https://github.com/isometimes/rpi4-osdev)