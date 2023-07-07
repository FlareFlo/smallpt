_Licensed under the MIT license_

# Installation

Stable rust toolchain from [the official installation guide](https://rustup.rs)

## Prerequisites

### WIndows
If the executable installer is not used, you may need to manually install [the MSVC prerequisites for Windows](https://rust-lang.github.io/rustup/installation/windows-msvc.html)

### Linux
The [GNU Compiler Collection tools](https://gcc.gnu.org/) must be installed, as Rust uses the linker from said toolchain.  
These can typically be installed through a package-manager:
* `build-essential` on ubuntu and debian
* `base-devel` on arch

# Rendering
## Run with optimal settings
```shell
RUSTFLAGS="--emit=asm -Ctarget-cpu=native" cargo run --release $SAMPLE_COUNT
```

## View image
Image viewers like Gwenview are required to view the PPM file format directly emitted  
If no such program is available, converting the image to alternative formats such as PNG are recommended
```shell
gwenview image.ppm
```


## Convert PPM to PNG using [ImageMagick](https://imagemagick.org/index.php)
ImageMagick is available for all operating systems
Converting to PNG usually cuts file size down by 60%
```shell
convert image.ppm image.png
```