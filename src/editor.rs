use crate::ElysieraParams;
use std::sync::Arc;
use nih_plug::editor::Editor;
use nih_plug_egui::{create_egui_editor, egui, widgets::ParamSlider, EguiState};

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(400, 700)
}

pub fn editor(
    params: Arc<ElysieraParams>
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        params.editor_state.clone(),
        (),
        |egui_ctx, _| {
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert(
                "Geist".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Geist-Regular.otf")),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "Geist".to_owned());

            egui_ctx.set_fonts(fonts);
        },
        move |egui_ctx, setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let params_list = vec![
                    ("Mix", vec![
                        ("Mix", &params.mix),
                        ("Pre Gain", &params.pre_gain),
                        ("Post Gain", &params.post_gain),
                    ]),

                    ("Reverb", vec![
                        ("Low Decay", &params.low_decay),
                        ("Low Crossover", &params.lf_crossover),
                        ("Mid Decay", &params.mid_decay),
                        ("High Filter Damping", &params.hf_damping),
                        ("Reverb Delay", &params.reverb_delay),
                        ("Reverb Mix", &params.reverb_mix),
                    ]),

                    ("Echo", vec![
                        ("Echo Delay", &params.delay),
                        ("Echo Feedback", &params.feedback),
                    ]),

                    ("Filter", vec![
                        ("Lowpass", &params.lowpass),
                        ("Lowpass Q", &params.lowpass_q),
                        ("Highpass", &params.highpass),
                        ("Highpass Q", &params.highpass_q),
                    ]),

                    ("Pitch", vec![
                        ("Pitch A", &params.pitch_a),
                        ("Pitch A Mix", &params.pitch_a_mix),
                        ("Pitch B", &params.pitch_b),
                        ("Pitch B Mix", &params.pitch_b_mix),
                    ]),

                    ("Modulate", vec![
                        ("Pitch Mod Rate", &params.pitch_mod_rate),
                        ("Pitch Mod Mix", &params.pitch_mod_mix),
                        ("Volume Mod Rate", &params.volume_mod_rate),
                        ("Volume Mod Mix", &params.volume_mod_mix),
                    ]),
                ];


                egui::Grid::new("params")
                    .striped(true)
                    .show(ui, |ui| {
                        params_list.iter().for_each(|(section, params)| {
                            ui.label(*section);
                            ui.end_row();

                            params.iter().for_each(|(name, param)| {
                                ui.label(*name);
                                ui.add(ParamSlider::for_param(*param, setter));
                                ui.end_row();
                            });
                        });
                    });
                // macro_rules! param {
                //     ($name:expr, $param:expr) => {
                //         ui.horizontal(|ui| {
                //             ui.label($name);
                //             ui.add(ParamSlider::for_param(&$param, setter));
                //         })
                //     };
                // }

                // ui.label("Mix");
                // param!("Mix", params.mix);
                // param!("Pre Gain", params.pre_gain);
                // param!("Post Gain", params.post_gain);

                // ui.label("Reverb");
                // param!("Low Decay", params.low_decay);
                // param!("Low Crossover", params.lf_crossover);
                // param!("Mid Decay", params.mid_decay);
                // param!("High Filter Damping", params.hf_damping);
                // param!("Reverb Delay", params.reverb_delay);
                // param!("Reverb Mix", params.reverb_mix);
            });
        }
    )
}