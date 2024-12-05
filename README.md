# Rust bindings to `dftd4` library

This crate performs safe wrapper on library `dftd4`.

This implementation based on C wrapper of original DFT-D4 [dftd4/dftd4](https://github.com/dftd4/dftd4).

## Usage

As an example, given molecular information, functional `SCAN` can be evaluated as
```rust
use rest_dftd4::prelude::*;
// if molecule but not periodic system, then `Some(&latice)` and `Some(&periodic)` can be both None
let structure = DFTD4Structure::new(natoms, &charges, &coords, Some(&latice), Some(&periodic));
let model = DFTD4Model::new(&structure);
let param = DFTD4Param::load_rational_damping("SCAN", true);
// gradient and sigma are optionally evaluated, controlled by last two boolean parameters
let (energy, gradient, sigma) = get_dispersion(&structure, &model, &param, true, true);
```

For details, we refer to [test case](tests/test.rs).

## Installation

### Shared library from conda-forge (recommended scheme)

The recommended installation scheme using by shared library:
- Make sure shared object files `libdftd4.so`, `libmctc-lib.so`, `libmulticharge.so`, `libblas.so`, `liblapack.so` are in environment variable `LD_LIBRARY_PATH`. These files can be obtained by conda/mamba installation (see also [dftd4 installation guide](https://github.com/dftd4/dftd4/?tab=readme-ov-file#conda-package)) and found in conda library list (usually in `<conda-base-path>/envs/<your-env>/lib/`).
- Then import this crate in your `Cargo.toml` file by 
    ```toml
    [dependencies]
    <...>
    rest_dftd4 = { version = "0.1" }
    ```

### Static library from conda-forge

For static library,
- Make sure shared object files `libdftd4.a`, `libmctc-lib.a`, `libmulticharge.a` are in environment variable `LD_LIBRARY_PATH`. These files can also be obtained by conda/mamba installation.
- Then import this crate in your `Cargo.toml` file by 
    ```toml
    [dependencies]
    <...>
    rest_dftd4 = { version = "0.1", features = ["static"] }
    ```
- Please also make sure dynamic libraries `gomp`, `gfortran`, `blas` and `lapack` are available in `LD_LIBRARY_PATH`.

Using static library have pros and cons:
- pro: It is more suitable for distribution, given the same architecture for compilation and usage.
- con: It is not fully static. It still links to external libraries `gomp`, `gfortran`, `blas` and `lapack`. In current workflow, you may need to provide them as shared libraries in `LD_LIBRARY_PATH`. These libraries are also provided by conda-forge.
- license note: `dftd4` library is LGPL-v3.0. Some restrictions may occur if you only distribute your program by static-linked binary if license of your program is not GPL-v3.

### Build both dftd4 and its rust bindings

We also provide automatic installation, if use have no dftd4 libraries at hand, and access to github.com is available. This is done by cmake compilation.

The following code may works:
```bash
sudo apt install liblapack-dev
git clone git@github.com:ajz34/tmp-dftd4-rs.git
cd tmp-dftd4-rs
cargo test
```
But if you also found that when incorporating this crate in other projects, it tells you `libdftd4.so`, `libmctc-lib.so` or `libmulticharge.so` not found; then you may try to find these shared objects in build directory, and add these libraries into `LD_LIBRARY_PATH`.

Also note that, DFT-D4 depends on BLAS and Lapack. When building by cmake, it will automatically check which Lapack to be used; and that Lapack is not promised to be `libblas.so`, but also be possible to be `libopenblas.so` or `libblis.so`.

## License

This project is dual licensed by Apache and MIT.

This project is simply wrapper. For guides to use DFT-D4, please refer to [dftd4/dftd4](https://github.com/dftd4/dftd4). Also note that original dftd4 library is licensed by LGPL-v3.
