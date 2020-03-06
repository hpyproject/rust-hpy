# HPy Rust bindings

This crate is a highly experimental work-in-progress support of
[HPy](https://github.com/pyhandle/hpy).

It could at some point be merged with the
[cpython](https://crates.io/crates/cpython) crate – or not.


## How to run the examples

1. Install `hpy`:

   ```
   git clone https://github.com/pyhandle/hpy
   cd hpy
   make
   ```

   at this point, there's a shared library in `hpy/universal`

2. Build and run an example:

   ```
   cd ../rust-hpy
   cargo build --example pof
   PYTHONPATH=../hpy python3 examples/load_pof.py
   ```
