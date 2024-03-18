declare name "elysiera";
declare version "1.0";

import("stdfaust.lib");

grp (x) = tgroup("", x);

// Mix

group_mix (x) = grp(hgroup("[0]Mix", x));
mix = group_mix(vslider("[0]Mix [style:knob] [unit:%]", 0.5, 0.0, 1.0, 0.01));
pre_gain = group_mix(vslider("[1]Pre Gain [style:knob] [unit:dB]", 0.0, -30.0, 10.0, 0.1));
post_gain = group_mix(vslider("[2]Post Gain [style:knob] [unit:dB]", 0.0, -30.0, 10.0, 0.1));

// - Reverb

group_reverb (x) = grp(hgroup("[1]Reverb", x));
// T60 = Time (in seconds) to decay 60dB in low-frequency band
low_decay = group_reverb(vslider("[0]Low decay [style:knob] [unit:s]", 3.7, 1.0, 60.0, 0.01));
// Crossover frequency (Hz) separating low and middle frequencies
low_crossover = group_reverb(vslider("[1]LF Crossover [style:knob] [unit:Hz]", 440.0, 50.0, 1000.0, 1));
// T60 = Time (in seconds) to decay 60dB in middle band
mid_decay = group_reverb(vslider("[2]Mid Decay [style:knob] [unit:s]", 4.68, 1.0, 60.0, 0.01));
// Frequency (Hz) at which the high-frequency T60 is half the middle-band's T60
high_filter_damping = group_reverb(vslider("[3]HF Damping [style:knob] [unit:Hz]", 8600.0, 1500.0, 0.49 * 44100.0, 1));
// Time before reverb hits
reverb_delay = group_reverb(vslider("[4]Reverb Delay [style:knob] [unit:ms]", 0.0, 0.0, 100.0, 0.01));
// Dry/wet mixer
reverb_mix = group_reverb(vslider("[5]Reverb Mix [style:knob] [unit:%]", 1.0, 0.0, 1.0, 0.01));
zita_rev = re.zita_rev1_stereo(
    reverb_delay, low_crossover, high_filter_damping, low_decay, mid_decay, 44100.0
);

// - Echo
group_echo (x) = grp(hgroup("[2]Echo", x));
// Echo delay time (in seconds)
echo_delay = group_echo(vslider("[0]Delay [style:knob] [unit:s]", 0.5, 0.0, 5.0, 0.01));
// Echo feedback (0.0 to 0.99)
echo_feedback = group_echo(vslider("[1]Feedback [style:knob] [unit:%]", 0.5, 0.0, 0.99, 0.01));

// - Pre-Filter

group_filter (x) = grp(hgroup("[3]Filter", x));
lp_filter_cutoff = group_filter(vslider("[0]Lowpass [style:knob] [unit:Hz]", 18000.0, 25.0, 22000.0, 0.01));
lp_filter_q = group_filter(vslider("[1]Lowpass Q [style:knob]", 0.71, 0.1, 2.0, 0.01));
hp_filter_cutoff = group_filter(vslider("[3]Highpass [style:knob] [unit:Hz]", 250.0, 25.0, 22000.0, 0.01));
hp_filter_q = group_filter(vslider("[4]Resonance [style:knob]", 0.71, 0.1, 2.0, 0.01));

// - Pitch

group_pitch (x) = grp(hgroup("[4]Pitch", x));
pitch_a = group_pitch(vslider("[0]Pitch A [style:knob] [unit:st]", 12.0, -12.0, 12.0, 1));
pitch_a_vol = group_pitch(vslider("[1]Pitch A Mix [style:knob] [unit:%]", 0.6, 0.0, 1.0, 0.01));
pitch_b = group_pitch(vslider("[1]Pitch B [style:knob] [unit:st]", 5.0, -12.0, 12.0, 1));
pitch_b_vol = group_pitch(vslider("[2]Pitch B Mix [style:knob] [unit:%]", 0.6, 0.0, 1.0, 0.01));

// - Mod

group_mod (x) = grp(hgroup("[5]Mod", x));
mod_hz = group_mod(vslider("[5]Mod Rate [style:knob] [unit:hz]", 3.1, 0.01, 10.0, 0.01));
mod_mix = group_mod(vslider("[6]Mod Mix [style:knob] [unit:%]", 1.0, 0.0, 1.0, 0.01));

c_samples = 2048;
c_xfade   = 1024;

process = _, _ :
    // Pre-gain
    par(i, 2, *(ba.db2linear(pre_gain)))
    : ef.dryWetMixerConstantPower(mix, (_ , _
        : par(i, 2,
            // Lowpass
            fi.svf.lp(lp_filter_cutoff, lp_filter_q)
            // Highpass
            : fi.svf.hp(hp_filter_cutoff, hp_filter_q)
            // Echo
            : ef.echo(5.0, echo_delay, echo_feedback)
        )
        : (
            // Make pitch_a appears on sin, pitch_b appears on cos and invert for other channel
            (_ <:
                ef.transpose(c_samples, c_xfade, pitch_a) * (os.oscsin(mod_hz) * mod_mix + (1 - mod_mix)) * pitch_a_vol
                + ef.transpose(c_samples, c_xfade, pitch_b) * (os.osccos(mod_hz) * mod_mix + (1 - mod_mix)) * pitch_b_vol
            ),
            (_ <:
                ef.transpose(c_samples, c_xfade, pitch_a) * (os.osccos(mod_hz) * mod_mix + (1 - mod_mix)) * pitch_a_vol
                + ef.transpose(c_samples, c_xfade, pitch_b) * (os.oscsin(mod_hz) * mod_mix + (1 - mod_mix)) * pitch_b_vol
            )
        )
        // Reverb
        :> ef.dryWetMixerConstantPower(reverb_mix, zita_rev)
    ))
    // Post-gain
    : par(i, 2, *(ba.db2linear(post_gain)));