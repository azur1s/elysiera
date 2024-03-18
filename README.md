# elysiera
Source code for Elysiera, a shimmer reverb plugin.

## Installing
WIP

## Contributing
I personally use [`debug.ps1`](debug.ps1) to build and automatically open a DAW for debugging purpose. Use it if it works.
```shell
# If you modified the faust file (faust/dsp.dsp) then you might have to
# regenerate both dsp.rs and params.rs
python faust2params.py faust/dsp.dsp

# Debug build, huge file size
cargo xtask bundle elysiera
# Release build, small file size
cargo xtask bundle elysiera --release
```