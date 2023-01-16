#![feature(let_chains)]

pub(crate) mod cli;
pub(crate) mod music;
pub(crate) mod synth;
pub(crate) mod upgrade;


use std::fs::File;
use std::io::Read;
use crate::cli::Cli;
use crate::music::{InstrChannel, Music, Tone};

fn main() {
	let music = File::open("auto_save.syb").map(|mut file| {
		Music::deserialize({
			let mut buf = vec![];
			file.read_to_end(&mut buf).expect("unable to read file");
			buf
	   })
	}).unwrap_or_else(|_| Music {
		bps: 8,
		section_height: 4,
		notes: vec![
			vec![Tone::empty();14];
			32
		],
		ic: vec![
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 1, volume: 255, enabled: true },
			InstrChannel { id: 2, volume: 74, enabled: true },
			InstrChannel { id: 2, volume: 74, enabled: true },
			InstrChannel { id: 3, volume: 74, enabled: true },
			InstrChannel { id: 3, volume: 74, enabled: true },
			InstrChannel { id: 4, volume: 191, enabled: true },
			InstrChannel { id: 4, volume: 191, enabled: true },
			InstrChannel { id: 4, volume: 191, enabled: true },
			InstrChannel { id: 4, volume: 191, enabled: true },
		]
	});
	Cli::start(music);
}

