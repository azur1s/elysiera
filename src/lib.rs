mod params;
mod dsp;
mod buffer;

use std::sync::Arc;
use nih_plug::prelude::*;
use faust_types::FaustDsp;

use params::*;
use dsp::ElysieraDSP;
use buffer::TempBuffer;

struct Elysiera {
    dsp: Box<ElysieraDSP>,
    params: Arc<ElysieraParams>,
    buffer: TempBuffer,
}

impl Default for Elysiera {
    fn default() -> Self {
        Self {
            dsp: Box::new(ElysieraDSP::new()),
            params: Arc::new(ElysieraParams::default()),
            buffer: TempBuffer::default(),
        }
    }
}

impl Plugin for Elysiera {
    const NAME: &'static str = "az.elysiera";
    const VENDOR: &'static str = "Azur1s";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "natapat.samutpong@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Sample rate: {}", buffer_config.sample_rate);
        self.dsp.init(buffer_config.sample_rate as i32);
        self.buffer.resize(2, 1024);
        true
    }

    fn reset(&mut self) {
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let count = buffer.samples() as i32;
        self.buffer.read_from_buffer(buffer);
        let output = buffer.as_slice();

        self.params.dsp_set_params(&mut self.dsp);

        self.dsp
            .compute(count, &self.buffer.slice2d(), output);

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Elysiera {
    const CLAP_ID: &'static str = "moe.azur.elysiera";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Elysiera {
    const VST3_CLASS_ID: [u8; 16] = *b"az.elysiera_____";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(Elysiera);
nih_export_vst3!(Elysiera);