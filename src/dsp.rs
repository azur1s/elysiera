mod dsp {
#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use faust_types::*;
/* ------------------------------------------------------------
name: "elysiera"
version: "1.0"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct mydspSIG0 {
	iVec0: [i32;2],
	iRec0: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec0[l1 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[f32]) {
		for i1 in 0..count {
			self.iVec0[0] = 1;
			self.iRec0[0] = (i32::wrapping_add(self.iVec0[1], self.iRec0[1])) % 65536;
			table[i1 as usize] = f32::cos(9.58738e-05 * (self.iRec0[0]) as f32);
			self.iVec0[1] = self.iVec0[0];
			self.iRec0[1] = self.iRec0[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec0: [0;2],
		iRec0: [0;2],
	}
}

pub struct mydspSIG1 {
	iVec2: [i32;2],
	iRec11: [i32;2],
}

impl mydspSIG1 {
	
	fn get_num_inputsmydspSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG1(&mut self, sample_rate: i32) {
		for l10 in 0..2 {
			self.iVec2[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec11[l11 as usize] = 0;
		}
	}
	
	fn fillmydspSIG1(&mut self, count: i32, table: &mut[f32]) {
		for i2 in 0..count {
			self.iVec2[0] = 1;
			self.iRec11[0] = (i32::wrapping_add(self.iVec2[1], self.iRec11[1])) % 65536;
			table[i2 as usize] = f32::sin(9.58738e-05 * (self.iRec11[0]) as f32);
			self.iVec2[1] = self.iVec2[0];
			self.iRec11[1] = self.iRec11[0];
		}
	}

}


pub fn newmydspSIG1() -> mydspSIG1 { 
	mydspSIG1 {
		iVec2: [0;2],
		iRec11: [0;2],
	}
}
static mut ftbl0mydspSIG0: [f32;65536] = [0.0;65536];
static mut ftbl1mydspSIG1: [f32;65536] = [0.0;65536];
fn mydsp_faustpower2_f(value: f32) -> f32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct mydsp {
	fVslider0: f32,
	fVslider1: f32,
	iVec1: [i32;2],
	fVslider2: f32,
	fSampleRate: i32,
	fConst0: f32,
	fConst1: f32,
	fRec1: [f32;2],
	fVslider3: f32,
	fVslider4: f32,
	fRec2: [f32;2],
	fVslider5: f32,
	fConst2: f32,
	fVslider6: f32,
	fRec8: [f32;2],
	fRec9: [f32;2],
	fVslider7: f32,
	fVslider8: f32,
	fRec4: [f32;2],
	fRec5: [f32;2],
	IOTA0: i32,
	fConst3: f32,
	fVslider9: f32,
	fVslider10: f32,
	fRec3: [f32;1048576],
	fVslider11: f32,
	fVslider12: f32,
	fRec12: [f32;2],
	fVslider13: f32,
	fVec3: [f32;16384],
	fVslider14: f32,
	fVslider15: f32,
	fConst5: f32,
	fVslider16: f32,
	fConst6: f32,
	fVslider17: f32,
	fRec24: [f32;2],
	fVslider18: f32,
	fRec23: [f32;2],
	fVec4: [f32;16384],
	iConst8: i32,
	fVslider19: f32,
	fConst9: f32,
	fVec5: [f32;2048],
	iConst10: i32,
	fRec21: [f32;2],
	fConst12: f32,
	fRec28: [f32;2],
	fRec27: [f32;2],
	fVec6: [f32;16384],
	iConst14: i32,
	fVec7: [f32;2048],
	iConst15: i32,
	fRec25: [f32;2],
	fConst17: f32,
	fRec32: [f32;2],
	fRec31: [f32;2],
	fVec8: [f32;16384],
	iConst19: i32,
	fVec9: [f32;4096],
	iConst20: i32,
	fRec29: [f32;2],
	fConst22: f32,
	fRec36: [f32;2],
	fRec35: [f32;2],
	fVec10: [f32;16384],
	iConst24: i32,
	fVec11: [f32;2048],
	iConst25: i32,
	fRec33: [f32;2],
	fConst27: f32,
	fRec40: [f32;2],
	fRec39: [f32;2],
	fVec12: [f32;32768],
	iConst29: i32,
	fRec46: [f32;2],
	fRec47: [f32;2],
	fRec42: [f32;2],
	fRec43: [f32;2],
	fRec41: [f32;1048576],
	fVec13: [f32;16384],
	fVec14: [f32;4096],
	iConst30: i32,
	fRec37: [f32;2],
	fConst32: f32,
	fRec52: [f32;2],
	fRec51: [f32;2],
	fVec15: [f32;16384],
	iConst34: i32,
	fVec16: [f32;4096],
	iConst35: i32,
	fRec49: [f32;2],
	fConst37: f32,
	fRec56: [f32;2],
	fRec55: [f32;2],
	fVec17: [f32;32768],
	iConst39: i32,
	fVec18: [f32;4096],
	iConst40: i32,
	fRec53: [f32;2],
	fConst42: f32,
	fRec60: [f32;2],
	fRec59: [f32;2],
	fVec19: [f32;32768],
	iConst44: i32,
	fVec20: [f32;2048],
	iConst45: i32,
	fRec57: [f32;2],
	fRec13: [f32;3],
	fRec14: [f32;3],
	fRec15: [f32;3],
	fRec16: [f32;3],
	fRec17: [f32;3],
	fRec18: [f32;3],
	fRec19: [f32;3],
	fRec20: [f32;3],
	fVslider20: f32,
}

impl FaustDsp for mydsp {
	type T = f32;
		
	fn new() -> mydsp { 
		mydsp {
			fVslider0: 0.0,
			fVslider1: 0.0,
			iVec1: [0;2],
			fVslider2: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fRec1: [0.0;2],
			fVslider3: 0.0,
			fVslider4: 0.0,
			fRec2: [0.0;2],
			fVslider5: 0.0,
			fConst2: 0.0,
			fVslider6: 0.0,
			fRec8: [0.0;2],
			fRec9: [0.0;2],
			fVslider7: 0.0,
			fVslider8: 0.0,
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			IOTA0: 0,
			fConst3: 0.0,
			fVslider9: 0.0,
			fVslider10: 0.0,
			fRec3: [0.0;1048576],
			fVslider11: 0.0,
			fVslider12: 0.0,
			fRec12: [0.0;2],
			fVslider13: 0.0,
			fVec3: [0.0;16384],
			fVslider14: 0.0,
			fVslider15: 0.0,
			fConst5: 0.0,
			fVslider16: 0.0,
			fConst6: 0.0,
			fVslider17: 0.0,
			fRec24: [0.0;2],
			fVslider18: 0.0,
			fRec23: [0.0;2],
			fVec4: [0.0;16384],
			iConst8: 0,
			fVslider19: 0.0,
			fConst9: 0.0,
			fVec5: [0.0;2048],
			iConst10: 0,
			fRec21: [0.0;2],
			fConst12: 0.0,
			fRec28: [0.0;2],
			fRec27: [0.0;2],
			fVec6: [0.0;16384],
			iConst14: 0,
			fVec7: [0.0;2048],
			iConst15: 0,
			fRec25: [0.0;2],
			fConst17: 0.0,
			fRec32: [0.0;2],
			fRec31: [0.0;2],
			fVec8: [0.0;16384],
			iConst19: 0,
			fVec9: [0.0;4096],
			iConst20: 0,
			fRec29: [0.0;2],
			fConst22: 0.0,
			fRec36: [0.0;2],
			fRec35: [0.0;2],
			fVec10: [0.0;16384],
			iConst24: 0,
			fVec11: [0.0;2048],
			iConst25: 0,
			fRec33: [0.0;2],
			fConst27: 0.0,
			fRec40: [0.0;2],
			fRec39: [0.0;2],
			fVec12: [0.0;32768],
			iConst29: 0,
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fRec41: [0.0;1048576],
			fVec13: [0.0;16384],
			fVec14: [0.0;4096],
			iConst30: 0,
			fRec37: [0.0;2],
			fConst32: 0.0,
			fRec52: [0.0;2],
			fRec51: [0.0;2],
			fVec15: [0.0;16384],
			iConst34: 0,
			fVec16: [0.0;4096],
			iConst35: 0,
			fRec49: [0.0;2],
			fConst37: 0.0,
			fRec56: [0.0;2],
			fRec55: [0.0;2],
			fVec17: [0.0;32768],
			iConst39: 0,
			fVec18: [0.0;4096],
			iConst40: 0,
			fRec53: [0.0;2],
			fConst42: 0.0,
			fRec60: [0.0;2],
			fRec59: [0.0;2],
			fVec19: [0.0;32768],
			iConst44: 0,
			fVec20: [0.0;2048],
			iConst45: 0,
			fRec57: [0.0;2],
			fRec13: [0.0;3],
			fRec14: [0.0;3],
			fRec15: [0.0;3],
			fRec16: [0.0;3],
			fRec17: [0.0;3],
			fRec18: [0.0;3],
			fRec19: [0.0;3],
			fRec20: [0.0;3],
			fVslider20: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/svf:author", r"Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", r"Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("misceffects.lib/dryWetMixerConstantPower:author", r"David Braun, revised by Stéphane Letz");
		m.declare("misceffects.lib/echo:author", r"Romain Michon");
		m.declare("misceffects.lib/name", r"Misc Effects Library");
		m.declare("misceffects.lib/version", r"2.4.0");
		m.declare("name", r"elysiera");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.5.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.1");
		m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
		m.declare("version", r"1.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, unsafe { &mut ftbl0mydspSIG0 });
		let mut sig1: mydspSIG1 = newmydspSIG1();
		sig1.instance_initmydspSIG1(sample_rate);
		sig1.fillmydspSIG1(65536, unsafe { &mut ftbl1mydspSIG1 });
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 0.0;
		self.fVslider1 = 0.5;
		self.fVslider2 = 3.1;
		self.fVslider3 = 1.0;
		self.fVslider4 = 5.0;
		self.fVslider5 = 1.8e+04;
		self.fVslider6 = 0.71;
		self.fVslider7 = 2.5e+02;
		self.fVslider8 = 0.71;
		self.fVslider9 = 0.5;
		self.fVslider10 = 0.5;
		self.fVslider11 = 0.6;
		self.fVslider12 = 12.0;
		self.fVslider13 = 0.6;
		self.fVslider14 = 1.0;
		self.fVslider15 = 4.68;
		self.fVslider16 = 8.6e+03;
		self.fVslider17 = 4.4e+02;
		self.fVslider18 = 3.7;
		self.fVslider19 = 0.0;
		self.fVslider20 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l2 in 0..2 {
			self.iVec1[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec8[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec9[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec4[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec5[l8 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l9 in 0..1048576 {
			self.fRec3[l9 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec12[l12 as usize] = 0.0;
		}
		for l13 in 0..16384 {
			self.fVec3[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec24[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec23[l15 as usize] = 0.0;
		}
		for l16 in 0..16384 {
			self.fVec4[l16 as usize] = 0.0;
		}
		for l17 in 0..2048 {
			self.fVec5[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec21[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec28[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec27[l20 as usize] = 0.0;
		}
		for l21 in 0..16384 {
			self.fVec6[l21 as usize] = 0.0;
		}
		for l22 in 0..2048 {
			self.fVec7[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec25[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec32[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec31[l25 as usize] = 0.0;
		}
		for l26 in 0..16384 {
			self.fVec8[l26 as usize] = 0.0;
		}
		for l27 in 0..4096 {
			self.fVec9[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec29[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec36[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec35[l30 as usize] = 0.0;
		}
		for l31 in 0..16384 {
			self.fVec10[l31 as usize] = 0.0;
		}
		for l32 in 0..2048 {
			self.fVec11[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec33[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec40[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec39[l35 as usize] = 0.0;
		}
		for l36 in 0..32768 {
			self.fVec12[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec46[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec47[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec42[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec43[l40 as usize] = 0.0;
		}
		for l41 in 0..1048576 {
			self.fRec41[l41 as usize] = 0.0;
		}
		for l42 in 0..16384 {
			self.fVec13[l42 as usize] = 0.0;
		}
		for l43 in 0..4096 {
			self.fVec14[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec37[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec52[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec51[l46 as usize] = 0.0;
		}
		for l47 in 0..16384 {
			self.fVec15[l47 as usize] = 0.0;
		}
		for l48 in 0..4096 {
			self.fVec16[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec49[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec56[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec55[l51 as usize] = 0.0;
		}
		for l52 in 0..32768 {
			self.fVec17[l52 as usize] = 0.0;
		}
		for l53 in 0..4096 {
			self.fVec18[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec53[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec60[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec59[l56 as usize] = 0.0;
		}
		for l57 in 0..32768 {
			self.fVec19[l57 as usize] = 0.0;
		}
		for l58 in 0..2048 {
			self.fVec20[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec57[l59 as usize] = 0.0;
		}
		for l60 in 0..3 {
			self.fRec13[l60 as usize] = 0.0;
		}
		for l61 in 0..3 {
			self.fRec14[l61 as usize] = 0.0;
		}
		for l62 in 0..3 {
			self.fRec15[l62 as usize] = 0.0;
		}
		for l63 in 0..3 {
			self.fRec16[l63 as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec17[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec18[l65 as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fRec19[l66 as usize] = 0.0;
		}
		for l67 in 0..3 {
			self.fRec20[l67 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = f32::min(1.92e+05, f32::max(1.0, (self.fSampleRate) as f32));
		self.fConst1 = 1.0 / self.fConst0;
		self.fConst2 = 3.1415927 / self.fConst0;
		self.fConst3 = 5.0 * self.fConst0;
		let mut fConst4: f32 = f32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst5 = 6.9077554 * (fConst4 / self.fConst0);
		self.fConst6 = 6.2831855 / self.fConst0;
		let mut fConst7: f32 = f32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst8 = (f32::min(8192.0, f32::max(0.0, fConst4 - fConst7))) as i32;
		self.fConst9 = 0.001 * self.fConst0;
		self.iConst10 = (f32::min(1024.0, f32::max(0.0, fConst7 + -1.0))) as i32;
		let mut fConst11: f32 = f32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst12 = 6.9077554 * (fConst11 / self.fConst0);
		let mut fConst13: f32 = f32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst14 = (f32::min(8192.0, f32::max(0.0, fConst11 - fConst13))) as i32;
		self.iConst15 = (f32::min(1024.0, f32::max(0.0, fConst13 + -1.0))) as i32;
		let mut fConst16: f32 = f32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst17 = 6.9077554 * (fConst16 / self.fConst0);
		let mut fConst18: f32 = f32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst19 = (f32::min(8192.0, f32::max(0.0, fConst16 - fConst18))) as i32;
		self.iConst20 = (f32::min(2048.0, f32::max(0.0, fConst18 + -1.0))) as i32;
		let mut fConst21: f32 = f32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst22 = 6.9077554 * (fConst21 / self.fConst0);
		let mut fConst23: f32 = f32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst24 = (f32::min(8192.0, f32::max(0.0, fConst21 - fConst23))) as i32;
		self.iConst25 = (f32::min(1024.0, f32::max(0.0, fConst23 + -1.0))) as i32;
		let mut fConst26: f32 = f32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst27 = 6.9077554 * (fConst26 / self.fConst0);
		let mut fConst28: f32 = f32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst29 = (f32::min(16384.0, f32::max(0.0, fConst26 - fConst28))) as i32;
		self.iConst30 = (f32::min(2048.0, f32::max(0.0, fConst28 + -1.0))) as i32;
		let mut fConst31: f32 = f32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst32 = 6.9077554 * (fConst31 / self.fConst0);
		let mut fConst33: f32 = f32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst34 = (f32::min(8192.0, f32::max(0.0, fConst31 - fConst33))) as i32;
		self.iConst35 = (f32::min(2048.0, f32::max(0.0, fConst33 + -1.0))) as i32;
		let mut fConst36: f32 = f32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst37 = 6.9077554 * (fConst36 / self.fConst0);
		let mut fConst38: f32 = f32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst39 = (f32::min(16384.0, f32::max(0.0, fConst36 - fConst38))) as i32;
		self.iConst40 = (f32::min(2048.0, f32::max(0.0, fConst38 + -1.0))) as i32;
		let mut fConst41: f32 = f32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst42 = 6.9077554 * (fConst41 / self.fConst0);
		let mut fConst43: f32 = f32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst44 = (f32::min(16384.0, f32::max(0.0, fConst41 - fConst43))) as i32;
		self.iConst45 = (f32::min(1024.0, f32::max(0.0, fConst43 + -1.0))) as i32;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_tab_box("elysiera");
		ui_interface.declare(None, "0", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.declare(Some(ParamIndex(0)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(0)), "unit", "%");
		ui_interface.add_vertical_slider("Mix", ParamIndex(0), 0.5, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
		ui_interface.add_vertical_slider("Pre Gain", ParamIndex(1), 0.0, -3e+01, 1e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(2)), "unit", "dB");
		ui_interface.add_vertical_slider("Post Gain", ParamIndex(2), 0.0, -3e+01, 1e+01, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("Reverb");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.declare(Some(ParamIndex(3)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "s");
		ui_interface.add_vertical_slider("Low decay", ParamIndex(3), 3.7, 1.0, 8.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.declare(Some(ParamIndex(4)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "Hz");
		ui_interface.add_vertical_slider("LF Crossover", ParamIndex(4), 4.4e+02, 5e+01, 1e+03, 1.0);
		ui_interface.declare(Some(ParamIndex(5)), "2", "");
		ui_interface.declare(Some(ParamIndex(5)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "s");
		ui_interface.add_vertical_slider("Mid Decay", ParamIndex(5), 4.68, 1.0, 8.0, 0.01);
		ui_interface.declare(Some(ParamIndex(6)), "3", "");
		ui_interface.declare(Some(ParamIndex(6)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "Hz");
		ui_interface.add_vertical_slider("HF Damping", ParamIndex(6), 8.6e+03, 1.5e+03, 21609.0, 1.0);
		ui_interface.declare(Some(ParamIndex(7)), "4", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "ms");
		ui_interface.add_vertical_slider("Reverb Delay", ParamIndex(7), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(8)), "5", "");
		ui_interface.declare(Some(ParamIndex(8)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "%");
		ui_interface.add_vertical_slider("Reverb Mix", ParamIndex(8), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Echo");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.declare(Some(ParamIndex(9)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "s");
		ui_interface.add_vertical_slider("Delay", ParamIndex(9), 0.5, 0.0, 5.0, 0.01);
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "%");
		ui_interface.add_vertical_slider("Feedback", ParamIndex(10), 0.5, 0.0, 0.99, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.declare(Some(ParamIndex(11)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "Hz");
		ui_interface.add_vertical_slider("Lowpass", ParamIndex(11), 1.8e+04, 25.0, 2.2e+04, 0.01);
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.declare(Some(ParamIndex(12)), "style", "knob");
		ui_interface.add_vertical_slider("Lowpass Q", ParamIndex(12), 0.71, 0.1, 2.0, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "3", "");
		ui_interface.declare(Some(ParamIndex(13)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "Hz");
		ui_interface.add_vertical_slider("Highpass", ParamIndex(13), 2.5e+02, 25.0, 2.2e+04, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "4", "");
		ui_interface.declare(Some(ParamIndex(14)), "style", "knob");
		ui_interface.add_vertical_slider("Resonance", ParamIndex(14), 0.71, 0.1, 2.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Pitch");
		ui_interface.declare(Some(ParamIndex(15)), "0", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "st");
		ui_interface.add_vertical_slider("Pitch A", ParamIndex(15), 12.0, -12.0, 12.0, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "1", "");
		ui_interface.declare(Some(ParamIndex(16)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(16)), "unit", "%");
		ui_interface.add_vertical_slider("Pitch A Mix", ParamIndex(16), 0.6, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.declare(Some(ParamIndex(17)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "st");
		ui_interface.add_vertical_slider("Pitch B", ParamIndex(17), 5.0, -12.0, 12.0, 1.0);
		ui_interface.declare(Some(ParamIndex(18)), "2", "");
		ui_interface.declare(Some(ParamIndex(18)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(18)), "unit", "%");
		ui_interface.add_vertical_slider("Pitch B Mix", ParamIndex(18), 0.6, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Mod");
		ui_interface.declare(Some(ParamIndex(19)), "5", "");
		ui_interface.declare(Some(ParamIndex(19)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(19)), "unit", "hz");
		ui_interface.add_vertical_slider("Mod Rate", ParamIndex(19), 3.1, 0.01, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(20)), "6", "");
		ui_interface.declare(Some(ParamIndex(20)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(20)), "unit", "%");
		ui_interface.add_vertical_slider("Mod Mix", ParamIndex(20), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			1 => Some(self.fVslider0),
			0 => Some(self.fVslider1),
			10 => Some(self.fVslider10),
			18 => Some(self.fVslider11),
			15 => Some(self.fVslider12),
			16 => Some(self.fVslider13),
			8 => Some(self.fVslider14),
			5 => Some(self.fVslider15),
			6 => Some(self.fVslider16),
			4 => Some(self.fVslider17),
			3 => Some(self.fVslider18),
			7 => Some(self.fVslider19),
			19 => Some(self.fVslider2),
			2 => Some(self.fVslider20),
			20 => Some(self.fVslider3),
			17 => Some(self.fVslider4),
			11 => Some(self.fVslider5),
			12 => Some(self.fVslider6),
			13 => Some(self.fVslider7),
			14 => Some(self.fVslider8),
			9 => Some(self.fVslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			1 => { self.fVslider0 = value }
			0 => { self.fVslider1 = value }
			10 => { self.fVslider10 = value }
			18 => { self.fVslider11 = value }
			15 => { self.fVslider12 = value }
			16 => { self.fVslider13 = value }
			8 => { self.fVslider14 = value }
			5 => { self.fVslider15 = value }
			6 => { self.fVslider16 = value }
			4 => { self.fVslider17 = value }
			3 => { self.fVslider18 = value }
			7 => { self.fVslider19 = value }
			19 => { self.fVslider2 = value }
			2 => { self.fVslider20 = value }
			20 => { self.fVslider3 = value }
			17 => { self.fVslider4 = value }
			11 => { self.fVslider5 = value }
			12 => { self.fVslider6 = value }
			13 => { self.fVslider7 = value }
			14 => { self.fVslider8 = value }
			9 => { self.fVslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: f32 = f32::powf(1e+01, 0.05 * self.fVslider0);
		let mut fSlow1: f32 = 1.5707964 * self.fVslider1;
		let mut fSlow2: f32 = f32::cos(fSlow1) * fSlow0;
		let mut fSlow3: f32 = self.fConst1 * self.fVslider2;
		let mut fSlow4: f32 = self.fVslider3;
		let mut fSlow5: f32 = f32::powf(2.0, 0.083333336 * self.fVslider4);
		let mut fSlow6: f32 = f32::tan(self.fConst2 * self.fVslider5);
		let mut fSlow7: f32 = fSlow6 * (1.0 / self.fVslider6 + fSlow6) + 1.0;
		let mut fSlow8: f32 = 2.0 / fSlow7;
		let mut fSlow9: f32 = fSlow6 / fSlow7;
		let mut fSlow10: f32 = f32::tan(self.fConst2 * self.fVslider7);
		let mut fSlow11: f32 = 1.0 / self.fVslider8;
		let mut fSlow12: f32 = fSlow10 * (fSlow11 + fSlow10) + 1.0;
		let mut fSlow13: f32 = 2.0 / fSlow12;
		let mut fSlow14: f32 = fSlow10 / fSlow12;
		let mut fSlow15: f32 = 1.0 / fSlow12;
		let mut iSlow16: i32 = i32::wrapping_add((f32::min(self.fConst3, f32::max(0.0, self.fConst0 * self.fVslider9))) as i32, 1);
		let mut fSlow17: f32 = self.fVslider10;
		let mut fSlow18: f32 = self.fVslider11;
		let mut fSlow19: f32 = f32::powf(2.0, 0.083333336 * self.fVslider12);
		let mut fSlow20: f32 = self.fVslider13;
		let mut fSlow21: f32 = 1.5707964 * self.fVslider14;
		let mut fSlow22: f32 = 0.70710677 * f32::cos(fSlow21);
		let mut fSlow23: f32 = self.fVslider15;
		let mut fSlow24: f32 = f32::exp(-(self.fConst5 / fSlow23));
		let mut fSlow25: f32 = mydsp_faustpower2_f(fSlow24);
		let mut fSlow26: f32 = 1.0 - fSlow25;
		let mut fSlow27: f32 = f32::cos(self.fConst6 * self.fVslider16);
		let mut fSlow28: f32 = 1.0 - fSlow27 * fSlow25;
		let mut fSlow29: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow28) / mydsp_faustpower2_f(fSlow26) + -1.0));
		let mut fSlow30: f32 = fSlow28 / fSlow26;
		let mut fSlow31: f32 = fSlow30 - fSlow29;
		let mut fSlow32: f32 = 1.0 / f32::tan(self.fConst2 * self.fVslider17);
		let mut fSlow33: f32 = 1.0 - fSlow32;
		let mut fSlow34: f32 = 1.0 / (fSlow32 + 1.0);
		let mut fSlow35: f32 = self.fVslider18;
		let mut fSlow36: f32 = f32::exp(-(self.fConst5 / fSlow35)) / fSlow24 + -1.0;
		let mut fSlow37: f32 = fSlow24 * (fSlow29 + (1.0 - fSlow30));
		let mut iSlow38: i32 = (f32::min(8192.0, f32::max(0.0, self.fConst9 * self.fVslider19))) as i32;
		let mut fSlow39: f32 = f32::exp(-(self.fConst12 / fSlow23));
		let mut fSlow40: f32 = mydsp_faustpower2_f(fSlow39);
		let mut fSlow41: f32 = 1.0 - fSlow40;
		let mut fSlow42: f32 = 1.0 - fSlow40 * fSlow27;
		let mut fSlow43: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow42) / mydsp_faustpower2_f(fSlow41) + -1.0));
		let mut fSlow44: f32 = fSlow42 / fSlow41;
		let mut fSlow45: f32 = fSlow44 - fSlow43;
		let mut fSlow46: f32 = f32::exp(-(self.fConst12 / fSlow35)) / fSlow39 + -1.0;
		let mut fSlow47: f32 = fSlow39 * (fSlow43 + (1.0 - fSlow44));
		let mut fSlow48: f32 = f32::exp(-(self.fConst17 / fSlow23));
		let mut fSlow49: f32 = mydsp_faustpower2_f(fSlow48);
		let mut fSlow50: f32 = 1.0 - fSlow49;
		let mut fSlow51: f32 = 1.0 - fSlow27 * fSlow49;
		let mut fSlow52: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow51) / mydsp_faustpower2_f(fSlow50) + -1.0));
		let mut fSlow53: f32 = fSlow51 / fSlow50;
		let mut fSlow54: f32 = fSlow53 - fSlow52;
		let mut fSlow55: f32 = f32::exp(-(self.fConst17 / fSlow35)) / fSlow48 + -1.0;
		let mut fSlow56: f32 = fSlow48 * (fSlow52 + (1.0 - fSlow53));
		let mut fSlow57: f32 = f32::exp(-(self.fConst22 / fSlow23));
		let mut fSlow58: f32 = mydsp_faustpower2_f(fSlow57);
		let mut fSlow59: f32 = 1.0 - fSlow58;
		let mut fSlow60: f32 = 1.0 - fSlow27 * fSlow58;
		let mut fSlow61: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow60) / mydsp_faustpower2_f(fSlow59) + -1.0));
		let mut fSlow62: f32 = fSlow60 / fSlow59;
		let mut fSlow63: f32 = fSlow62 - fSlow61;
		let mut fSlow64: f32 = f32::exp(-(self.fConst22 / fSlow35)) / fSlow57 + -1.0;
		let mut fSlow65: f32 = fSlow57 * (fSlow61 + (1.0 - fSlow62));
		let mut fSlow66: f32 = f32::exp(-(self.fConst27 / fSlow23));
		let mut fSlow67: f32 = mydsp_faustpower2_f(fSlow66);
		let mut fSlow68: f32 = 1.0 - fSlow67;
		let mut fSlow69: f32 = 1.0 - fSlow27 * fSlow67;
		let mut fSlow70: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow69) / mydsp_faustpower2_f(fSlow68) + -1.0));
		let mut fSlow71: f32 = fSlow69 / fSlow68;
		let mut fSlow72: f32 = fSlow71 - fSlow70;
		let mut fSlow73: f32 = f32::exp(-(self.fConst27 / fSlow35)) / fSlow66 + -1.0;
		let mut fSlow74: f32 = fSlow66 * (fSlow70 + (1.0 - fSlow71));
		let mut fSlow75: f32 = f32::exp(-(self.fConst32 / fSlow23));
		let mut fSlow76: f32 = mydsp_faustpower2_f(fSlow75);
		let mut fSlow77: f32 = 1.0 - fSlow76;
		let mut fSlow78: f32 = 1.0 - fSlow27 * fSlow76;
		let mut fSlow79: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow78) / mydsp_faustpower2_f(fSlow77) + -1.0));
		let mut fSlow80: f32 = fSlow78 / fSlow77;
		let mut fSlow81: f32 = fSlow80 - fSlow79;
		let mut fSlow82: f32 = f32::exp(-(self.fConst32 / fSlow35)) / fSlow75 + -1.0;
		let mut fSlow83: f32 = fSlow75 * (fSlow79 + (1.0 - fSlow80));
		let mut fSlow84: f32 = f32::exp(-(self.fConst37 / fSlow23));
		let mut fSlow85: f32 = mydsp_faustpower2_f(fSlow84);
		let mut fSlow86: f32 = 1.0 - fSlow85;
		let mut fSlow87: f32 = 1.0 - fSlow27 * fSlow85;
		let mut fSlow88: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow87) / mydsp_faustpower2_f(fSlow86) + -1.0));
		let mut fSlow89: f32 = fSlow87 / fSlow86;
		let mut fSlow90: f32 = fSlow89 - fSlow88;
		let mut fSlow91: f32 = f32::exp(-(self.fConst37 / fSlow35)) / fSlow84 + -1.0;
		let mut fSlow92: f32 = fSlow84 * (fSlow88 + (1.0 - fSlow89));
		let mut fSlow93: f32 = f32::exp(-(self.fConst42 / fSlow23));
		let mut fSlow94: f32 = mydsp_faustpower2_f(fSlow93);
		let mut fSlow95: f32 = 1.0 - fSlow94;
		let mut fSlow96: f32 = 1.0 - fSlow27 * fSlow94;
		let mut fSlow97: f32 = f32::sqrt(f32::max(0.0, mydsp_faustpower2_f(fSlow96) / mydsp_faustpower2_f(fSlow95) + -1.0));
		let mut fSlow98: f32 = fSlow96 / fSlow95;
		let mut fSlow99: f32 = fSlow98 - fSlow97;
		let mut fSlow100: f32 = f32::exp(-(self.fConst42 / fSlow35)) / fSlow93 + -1.0;
		let mut fSlow101: f32 = fSlow93 * (fSlow97 + (1.0 - fSlow98));
		let mut fSlow102: f32 = 0.26162952 * f32::sin(fSlow21);
		let mut fSlow103: f32 = f32::sin(fSlow1);
		let mut fSlow104: f32 = 0.70710677 * f32::powf(1e+01, 0.05 * self.fVslider20);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: f32 = *input0;
			self.iVec1[0] = 1;
			let mut fTemp1: f32 = if i32::wrapping_sub(1, self.iVec1[1]) != 0 {0.0} else {fSlow3 + self.fRec1[1]};
			self.fRec1[0] = fTemp1 - f32::floor(fTemp1);
			let mut iTemp2: i32 = std::cmp::max(0, std::cmp::min((65536.0 * self.fRec1[0]) as i32, 65535));
			let mut fTemp3: f32 = 1.0 - fSlow4 * (1.0 - unsafe { ftbl0mydspSIG0[iTemp2 as usize] });
			self.fRec2[0] = (self.fRec2[1] + 2049.0 - fSlow5) % 2048.0;
			let mut fTemp4: f32 = f32::min(0.0009765625 * self.fRec2[0], 1.0);
			let mut fTemp5: f32 = 1.0 - fTemp4;
			let mut fTemp6: f32 = fSlow0 * fTemp0;
			let mut fTemp7: f32 = self.fRec8[1] + fSlow6 * (fTemp6 - self.fRec9[1]);
			self.fRec8[0] = fSlow8 * fTemp7 - self.fRec8[1];
			let mut fTemp8: f32 = self.fRec9[1] + fSlow9 * fTemp7;
			self.fRec9[0] = 2.0 * fTemp8 - self.fRec9[1];
			let mut fRec10: f32 = fTemp8;
			let mut fTemp9: f32 = self.fRec4[1] + fSlow10 * (fRec10 - self.fRec5[1]);
			self.fRec4[0] = fSlow13 * fTemp9 - self.fRec4[1];
			let mut fTemp10: f32 = self.fRec5[1] + fSlow14 * fTemp9;
			self.fRec5[0] = 2.0 * fTemp10 - self.fRec5[1];
			let mut fRec6: f32 = fSlow15 * fTemp9;
			let mut fRec7: f32 = fTemp10;
			self.fRec3[(self.IOTA0 & 1048575) as usize] = fRec10 + fSlow17 * self.fRec3[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 1048575) as usize] - (fRec7 + fSlow11 * fRec6);
			let mut fTemp11: f32 = self.fRec2[0] + 2048.0;
			let mut iTemp12: i32 = (fTemp11) as i32;
			let mut iTemp13: i32 = std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp12, 1)));
			let mut fTemp14: f32 = f32::floor(fTemp11);
			let mut fTemp15: f32 = self.fRec2[0] + (2048.0 - fTemp14);
			let mut fTemp16: f32 = fTemp14 + (-2047.0 - self.fRec2[0]);
			let mut iTemp17: i32 = std::cmp::min(65537, std::cmp::max(0, iTemp12));
			let mut iTemp18: i32 = (self.fRec2[0]) as i32;
			let mut iTemp19: i32 = std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp18, 1)));
			let mut fTemp20: f32 = f32::floor(self.fRec2[0]);
			let mut fTemp21: f32 = self.fRec2[0] - fTemp20;
			let mut fTemp22: f32 = fTemp20 + (1.0 - self.fRec2[0]);
			let mut iTemp23: i32 = std::cmp::min(65537, std::cmp::max(0, iTemp18));
			let mut fTemp24: f32 = 1.0 - fSlow4 * (1.0 - unsafe { ftbl1mydspSIG1[iTemp2 as usize] });
			self.fRec12[0] = (self.fRec12[1] + 2049.0 - fSlow19) % 2048.0;
			let mut fTemp25: f32 = f32::min(0.0009765625 * self.fRec12[0], 1.0);
			let mut fTemp26: f32 = 1.0 - fTemp25;
			let mut fTemp27: f32 = self.fRec12[0] + 2048.0;
			let mut iTemp28: i32 = (fTemp27) as i32;
			let mut iTemp29: i32 = std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp28, 1)));
			let mut fTemp30: f32 = f32::floor(fTemp27);
			let mut fTemp31: f32 = self.fRec12[0] + (2048.0 - fTemp30);
			let mut fTemp32: f32 = fTemp30 + (-2047.0 - self.fRec12[0]);
			let mut iTemp33: i32 = std::cmp::min(65537, std::cmp::max(0, iTemp28));
			let mut iTemp34: i32 = (self.fRec12[0]) as i32;
			let mut iTemp35: i32 = std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp34, 1)));
			let mut fTemp36: f32 = f32::floor(self.fRec12[0]);
			let mut fTemp37: f32 = self.fRec12[0] - fTemp36;
			let mut fTemp38: f32 = fTemp36 + (1.0 - self.fRec12[0]);
			let mut iTemp39: i32 = std::cmp::min(65537, std::cmp::max(0, iTemp34));
			let mut fTemp40: f32 = fSlow20 * ((self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp39)) & 1048575) as usize] * fTemp38 + fTemp37 * self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 1048575) as usize]) * fTemp25 + (self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp33)) & 1048575) as usize] * fTemp32 + fTemp31 * self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp29)) & 1048575) as usize]) * fTemp26) * fTemp24 + fSlow18 * ((self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp23)) & 1048575) as usize] * fTemp22 + fTemp21 * self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp19)) & 1048575) as usize]) * fTemp4 + (self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp17)) & 1048575) as usize] * fTemp16 + fTemp15 * self.fRec3[((i32::wrapping_sub(self.IOTA0, iTemp13)) & 1048575) as usize]) * fTemp5) * fTemp3;
			self.fVec3[(self.IOTA0 & 16383) as usize] = fTemp40;
			self.fRec24[0] = fSlow34 * (self.fRec17[1] + self.fRec17[2] - fSlow33 * self.fRec24[1]);
			self.fRec23[0] = fSlow37 * (self.fRec17[1] + fSlow36 * self.fRec24[0]) + fSlow31 * self.fRec23[1];
			self.fVec4[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec23[0] + 1e-20;
			let mut fTemp41: f32 = 0.3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 16383) as usize];
			let mut fTemp42: f32 = fTemp41 + self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst8)) & 16383) as usize] - 0.6 * self.fRec21[1];
			self.fVec5[(self.IOTA0 & 2047) as usize] = fTemp42;
			self.fRec21[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst10)) & 2047) as usize];
			let mut fRec22: f32 = 0.6 * fTemp42;
			self.fRec28[0] = fSlow34 * (self.fRec13[1] + self.fRec13[2] - fSlow33 * self.fRec28[1]);
			self.fRec27[0] = fSlow47 * (self.fRec13[1] + fSlow46 * self.fRec28[0]) + fSlow45 * self.fRec27[1];
			self.fVec6[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec27[0] + 1e-20;
			let mut fTemp43: f32 = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst14)) & 16383) as usize] + fTemp41 - 0.6 * self.fRec25[1];
			self.fVec7[(self.IOTA0 & 2047) as usize] = fTemp43;
			self.fRec25[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst15)) & 2047) as usize];
			let mut fRec26: f32 = 0.6 * fTemp43;
			let mut fTemp44: f32 = fRec26 + fRec22;
			self.fRec32[0] = -(fSlow34 * (fSlow33 * self.fRec32[1] - (self.fRec15[1] + self.fRec15[2])));
			self.fRec31[0] = fSlow56 * (self.fRec15[1] + fSlow55 * self.fRec32[0]) + fSlow54 * self.fRec31[1];
			self.fVec8[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec31[0] + 1e-20;
			let mut fTemp45: f32 = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst19)) & 16383) as usize] - (fTemp41 + 0.6 * self.fRec29[1]);
			self.fVec9[(self.IOTA0 & 4095) as usize] = fTemp45;
			self.fRec29[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst20)) & 4095) as usize];
			let mut fRec30: f32 = 0.6 * fTemp45;
			self.fRec36[0] = fSlow34 * (self.fRec19[1] + self.fRec19[2] - fSlow33 * self.fRec36[1]);
			self.fRec35[0] = fSlow65 * (self.fRec19[1] + fSlow64 * self.fRec36[0]) + fSlow63 * self.fRec35[1];
			self.fVec10[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec35[0] + 1e-20;
			let mut fTemp46: f32 = self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize] - (fTemp41 + 0.6 * self.fRec33[1]);
			self.fVec11[(self.IOTA0 & 2047) as usize] = fTemp46;
			self.fRec33[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst25)) & 2047) as usize];
			let mut fRec34: f32 = 0.6 * fTemp46;
			let mut fTemp47: f32 = fRec34 + fRec30 + fTemp44;
			self.fRec40[0] = -(fSlow34 * (fSlow33 * self.fRec40[1] - (self.fRec14[1] + self.fRec14[2])));
			self.fRec39[0] = fSlow74 * (self.fRec14[1] + fSlow73 * self.fRec40[0]) + fSlow72 * self.fRec39[1];
			self.fVec12[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec39[0] + 1e-20;
			let mut fTemp48: f32 = *input1;
			let mut fTemp49: f32 = fSlow0 * fTemp48;
			let mut fTemp50: f32 = self.fRec46[1] + fSlow6 * (fTemp49 - self.fRec47[1]);
			self.fRec46[0] = fSlow8 * fTemp50 - self.fRec46[1];
			let mut fTemp51: f32 = self.fRec47[1] + fSlow9 * fTemp50;
			self.fRec47[0] = 2.0 * fTemp51 - self.fRec47[1];
			let mut fRec48: f32 = fTemp51;
			let mut fTemp52: f32 = self.fRec42[1] + fSlow10 * (fRec48 - self.fRec43[1]);
			self.fRec42[0] = fSlow13 * fTemp52 - self.fRec42[1];
			let mut fTemp53: f32 = self.fRec43[1] + fSlow14 * fTemp52;
			self.fRec43[0] = 2.0 * fTemp53 - self.fRec43[1];
			let mut fRec44: f32 = fSlow15 * fTemp52;
			let mut fRec45: f32 = fTemp53;
			self.fRec41[(self.IOTA0 & 1048575) as usize] = fRec48 + fSlow17 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 1048575) as usize] - (fRec45 + fSlow11 * fRec44);
			let mut fTemp54: f32 = fSlow20 * fTemp3 * (fTemp25 * (fTemp38 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp39)) & 1048575) as usize] + fTemp37 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 1048575) as usize]) + fTemp26 * (fTemp32 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp33)) & 1048575) as usize] + fTemp31 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp29)) & 1048575) as usize])) + fSlow18 * fTemp24 * (fTemp4 * (fTemp22 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp23)) & 1048575) as usize] + fTemp21 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp19)) & 1048575) as usize]) + fTemp5 * (fTemp16 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp17)) & 1048575) as usize] + fTemp15 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp13)) & 1048575) as usize]));
			self.fVec13[(self.IOTA0 & 16383) as usize] = fTemp54;
			let mut fTemp55: f32 = 0.3 * self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 16383) as usize];
			let mut fTemp56: f32 = fTemp55 + 0.6 * self.fRec37[1] + self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst29)) & 32767) as usize];
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp56;
			self.fRec37[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst30)) & 4095) as usize];
			let mut fRec38: f32 = -(0.6 * fTemp56);
			self.fRec52[0] = -(fSlow34 * (fSlow33 * self.fRec52[1] - (self.fRec18[1] + self.fRec18[2])));
			self.fRec51[0] = fSlow83 * (self.fRec18[1] + fSlow82 * self.fRec52[0]) + fSlow81 * self.fRec51[1];
			self.fVec15[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec51[0] + 1e-20;
			let mut fTemp57: f32 = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst34)) & 16383) as usize] + fTemp55 + 0.6 * self.fRec49[1];
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp57;
			self.fRec49[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst35)) & 4095) as usize];
			let mut fRec50: f32 = -(0.6 * fTemp57);
			self.fRec56[0] = -(fSlow34 * (fSlow33 * self.fRec56[1] - (self.fRec16[1] + self.fRec16[2])));
			self.fRec55[0] = fSlow92 * (self.fRec16[1] + fSlow91 * self.fRec56[0]) + fSlow90 * self.fRec55[1];
			self.fVec17[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec55[0] + 1e-20;
			let mut fTemp58: f32 = 0.6 * self.fRec53[1] + self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst39)) & 32767) as usize];
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp58 - fTemp55;
			self.fRec53[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst40)) & 4095) as usize];
			let mut fRec54: f32 = 0.6 * (fTemp55 - fTemp58);
			self.fRec60[0] = fSlow34 * (self.fRec20[1] + self.fRec20[2] - fSlow33 * self.fRec60[1]);
			self.fRec59[0] = fSlow101 * (self.fRec20[1] + fSlow100 * self.fRec60[0]) + fSlow99 * self.fRec59[1];
			self.fVec19[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec59[0] + 1e-20;
			let mut fTemp59: f32 = 0.6 * self.fRec57[1] + self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst44)) & 32767) as usize];
			self.fVec20[(self.IOTA0 & 2047) as usize] = fTemp59 - fTemp55;
			self.fRec57[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst45)) & 2047) as usize];
			let mut fRec58: f32 = 0.6 * (fTemp55 - fTemp59);
			self.fRec13[0] = self.fRec57[1] + self.fRec53[1] + self.fRec49[1] + self.fRec37[1] + self.fRec33[1] + self.fRec29[1] + self.fRec21[1] + self.fRec25[1] + fRec58 + fRec54 + fRec50 + fRec38 + fTemp47;
			self.fRec14[0] = self.fRec33[1] + self.fRec29[1] + self.fRec21[1] + self.fRec25[1] + fTemp47 - (self.fRec57[1] + self.fRec53[1] + self.fRec49[1] + self.fRec37[1] + fRec58 + fRec54 + fRec38 + fRec50);
			let mut fTemp60: f32 = fRec30 + fRec34;
			self.fRec15[0] = self.fRec49[1] + self.fRec37[1] + self.fRec21[1] + self.fRec25[1] + fRec50 + fRec38 + fTemp44 - (self.fRec57[1] + self.fRec53[1] + self.fRec33[1] + self.fRec29[1] + fRec58 + fRec54 + fTemp60);
			self.fRec16[0] = self.fRec57[1] + self.fRec53[1] + self.fRec21[1] + self.fRec25[1] + fRec58 + fRec54 + fTemp44 - (self.fRec49[1] + self.fRec37[1] + self.fRec33[1] + self.fRec29[1] + fRec50 + fRec38 + fTemp60);
			let mut fTemp61: f32 = fRec22 + fRec34;
			let mut fTemp62: f32 = fRec26 + fRec30;
			self.fRec17[0] = self.fRec53[1] + self.fRec37[1] + self.fRec29[1] + self.fRec25[1] + fRec54 + fRec38 + fTemp62 - (self.fRec57[1] + self.fRec49[1] + self.fRec33[1] + self.fRec21[1] + fRec58 + fRec50 + fTemp61);
			self.fRec18[0] = self.fRec57[1] + self.fRec49[1] + self.fRec29[1] + self.fRec25[1] + fRec58 + fRec50 + fTemp62 - (self.fRec53[1] + self.fRec37[1] + self.fRec33[1] + self.fRec21[1] + fRec54 + fRec38 + fTemp61);
			let mut fTemp63: f32 = fRec22 + fRec30;
			let mut fTemp64: f32 = fRec26 + fRec34;
			self.fRec19[0] = self.fRec57[1] + self.fRec37[1] + self.fRec33[1] + self.fRec25[1] + fRec58 + fRec38 + fTemp64 - (self.fRec53[1] + self.fRec49[1] + self.fRec29[1] + self.fRec21[1] + fRec54 + fRec50 + fTemp63);
			self.fRec20[0] = self.fRec53[1] + self.fRec49[1] + self.fRec33[1] + self.fRec25[1] + fRec54 + fRec50 + fTemp64 - (self.fRec57[1] + self.fRec37[1] + self.fRec29[1] + self.fRec21[1] + fRec58 + fRec38 + fTemp63);
			*output0 = fSlow104 * (fSlow103 * (fSlow102 * (self.fRec14[0] + self.fRec15[0]) + fSlow22 * fTemp40) + fSlow2 * fTemp0);
			*output1 = fSlow104 * (fSlow103 * (fSlow102 * (self.fRec14[0] - self.fRec15[0]) + fSlow22 * fTemp54) + fSlow2 * fTemp48);
			self.iVec1[1] = self.iVec1[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec12[1] = self.fRec12[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[2] = self.fRec14[1];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[2] = self.fRec15[1];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[2] = self.fRec18[1];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[2] = self.fRec19[1];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[2] = self.fRec20[1];
			self.fRec20[1] = self.fRec20[0];
		}
	}

}

}
pub use dsp::mydsp as ElysieraDSP;