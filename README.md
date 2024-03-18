# elysiera
Source code for Elysiera, a shimmer reverb plugin.

## How
Basically I was bored and stumbled upon [Eno/Lanois Shimmer Sound: How it is made](https://valhalladsp.wordpress.com/2010/05/11/enolanois-shimmer-sound-how-it-is-made/) by ValhallaDSP and I want to try it out.

> *"The basic foundation of the Brian Eno / Daniel Lanois shimmer sound is fairly simple: Create a feedback loop, incorporating a pitch shifter set to +1 octave, and a reverb with a fairly long decay time."*

And a bunch more googling led me to believe that shimmer reverb is just reverb with pitch-shifting and fancy stuff, so I did just that.

The [DSP file](faust/dsp.dsp) is pretty self-explanatory on how it works:
1. Filter signal with low and high pass.
2. Echo because why not.
3. Run it through 2 pitch shifters.
4. Modulate first pitch shifter with sine oscillator and second with cosine oscillator so that it have that wavy sound. Do the opposite for the right channel because why not.
5. Run it through reverb (I use Fons Adriaensen's `zita-rev1` because it sounded cool).


****
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