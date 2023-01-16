use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::music::Music;
use crate::synth::create_instrument;
use crate::synth::synth_source::SynthSource;
use crate::synth::waves::SAMPLE_RATE;

// http://soundfile.sapp.org/doc/WaveFormat/
pub(crate) fn export_wav<P: AsRef<Path>>(music: &Music, path: P) {
    let sound_data = {
        let mut data = vec![];
        let mut channels = vec![];
        for c in &music.ic {
            let instr = create_instrument(c.id);
            channels.push(SynthSource::create(instr.synth));
        }
        let tick = (SAMPLE_RATE / music.bps as f32) as usize;
        let three_quarter_tick = tick * 3 / 4;
        println!();
        for ct in 0..music.size().1 {
            for (i, c) in channels.iter_mut().enumerate() {
                let mut input = c.1.lock().unwrap();
                input.playing = music.at(i, ct).note.map_or_else(||{
                    None
                }, |note| {
                    input.freq = note.frequency();
                    Some(())
                }).is_some();
            }
            for t in 0..tick {
                let mut s = 0.0;
                for (i, c) in channels.iter_mut().enumerate() {
                    s += c.0.next().unwrap() * music.ic[i].volume as f32;
                    if t == three_quarter_tick && music.at(i, ct).note.map(|note| note.short).unwrap_or(false) {
                        c.1.lock().unwrap().playing = false
                    }
                }
                data.push(((s / 255.0 / channels.len() as f32 + 1.0) as f32 * 127.5) as u8);
            }
            println!("\x1b[1Fexporting {:3.1}%", ct as f32 / music.size().1 as f32 * 100.0 as f32)
        }
        println!("\x1b[1F               ");
        data
    };
    let bits_per_sample = 8u16;

    let sub_chunk2_size = (sound_data.len() * 1 * bits_per_sample as usize / 8) as u32;

    let mut wav_file = File::create(path).expect("couldnt create wav export file");
    wav_file.write_all(b"RIFF").unwrap();
    wav_file.write_all(&(sub_chunk2_size + 36).to_le_bytes()).unwrap();  // 36 + SubChunk2Size == 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size)
    wav_file.write_all(b"WAVE").unwrap();
    // sub chunk 1
    wav_file.write_all(b"fmt ").unwrap();
    wav_file.write_all(&16u32.to_le_bytes()).unwrap();  // size
    wav_file.write_all(&1u16.to_le_bytes()).unwrap();   // audio format
    wav_file.write_all(&1u16.to_le_bytes()).unwrap();   // num channels
    wav_file.write_all(&(SAMPLE_RATE as u32).to_le_bytes()).unwrap();  // sample rate
    wav_file.write_all(&(SAMPLE_RATE as u32 * 1 * bits_per_sample as u32 / 8).to_le_bytes()).unwrap();  // byte rate == SampleRate * NumChannels * BitsPerSample/8
    wav_file.write_all(&(1 * bits_per_sample as u16 / 8).to_le_bytes()).unwrap();   // BlockAlign == NumChannels * BitsPerSample/8
    wav_file.write_all(&bits_per_sample.to_le_bytes()).unwrap();                    // BitsPerSample
    // sub chunk 2
    wav_file.write_all(b"data").unwrap();
    wav_file.write_all(&sub_chunk2_size.to_le_bytes()).unwrap();  // Subchunk2Size == NumSamples * NumChannels * BitsPerSample/8
    wav_file.write_all(sound_data.as_slice()).unwrap();
}