#![feature(let_chains)]

pub(crate) mod cli;
pub(crate) mod music;
pub(crate) mod synth;


use std::fs::File;
use std::io::Read;
use crate::cli::Cli;
use crate::music::{Music, Tone};

fn main() {
	let music = File::open("auto_save.syb").map(|mut file| {
		Music::deserialize({
			let mut buf = vec![];
			file.read_to_end(&mut buf).expect("unable to read file");
			&mut buf.into_iter()
	   })
	}).unwrap_or_else(|_| Music {
		notes: vec![
			[Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty(), Tone::empty() ,Tone::empty(), Tone::empty()];
			32
		],
	});
	Cli::start(music);
}
