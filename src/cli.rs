
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::process::{exit};
use std::sync::{Arc, Mutex};
use std::{thread, usize};
use std::time::{SystemTime};
use getch::Getch;
use rodio::{OutputStream, Sink};

use crate::music::{Music, Note, Tone};
use crate::synth::channel::SynthChannel;
use crate::synth::waves::{SawSynth, SineSynth, SquareSynth, TriangleSynth};
use crate::synth::synth_source::SynthSource;

pub(crate) struct Cli {
    music: Music,
    input: Arc<Mutex<VecDeque<u8>>>,
    cursor: (i32, i32),
    viewport: i32,
    is_playing: bool,
    play_cursor: i32,
    viewport_height: i32,
    next_note_time: u128,
    _stream: OutputStream,
    channels: [SynthChannel;14],
    key_macro: Vec<u8>,
}

impl Cli {
    pub(crate) fn start(music: Music) {
        print!("\x1b[?25l");  // hide cursor
        print!("\x1b[2J");  // erase screen
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut cli = Cli {
            music,
            input: Default::default(),
            cursor: (0, 0),
            viewport: 0,
            is_playing: false,
            play_cursor: 0,
            next_note_time: 0,
            viewport_height: 32,
            _stream,
            key_macro: vec![],
            channels: unsafe {
                #[allow(invalid_value, deprecated)]
                let mut arr: [SynthChannel; 14] = std::mem::uninitialized();
                for (item, inp) in arr.iter_mut().zip(
                    [
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SineSynth), 1.2),
                        (SynthSource::create(SquareSynth), 0.35),
                        (SynthSource::create(SawSynth), 0.35),
                        (SynthSource::create(SquareSynth), 0.35),
                        (SynthSource::create(SawSynth), 0.35),
                        (SynthSource::create(TriangleSynth), 0.9),
                        (SynthSource::create(TriangleSynth), 0.9),
                        (SynthSource::create(TriangleSynth), 0.9),
                        (SynthSource::create(TriangleSynth), 0.9),
                    ]) {
                    let ((src, input), volume) = inp;
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.set_volume(0.4 * volume);
                    sink.append(src);
                    let sc = SynthChannel {
                        enabled: true,
                        sink,
                        input
                    };
                    std::ptr::write(item, sc);
                }
                arr
            }
        };
        let thread_in = cli.input.clone();
        thread::spawn(move ||{
            let getch = Getch::new();
            loop {
                let ch = getch.getch().unwrap();
                thread_in.lock().unwrap().push_back(ch);
            }
        });
        cli.run();
    }

    fn run(&mut self){
        loop {
            if let Some(c) = self.input() {
                self.handle_input(c)
            }
            self.handle_playback();
            // move to top left, print
            print!("\x1b[H{}", self.render())
        }
    }

    fn handle_input(&mut self, c: u8){
        match c {
            27 => self.quit(),  // esc
            224 => match self.await_input() {  // movement
                72 => self.move_cursor_by(0, -1),  // up
                80 => self.move_cursor_by(0, 1),   // down
                75 => self.move_cursor_by(-1, 0),  // left
                77 => self.move_cursor_by(1, 0),   // right

                71 => self.move_cursor_to(0, self.cursor.1),                               // start
                79 => self.move_cursor_to(self.music.size().0 as i32, self.cursor.1),      // end
                73 => self.move_cursor_by(self.cursor.0, -self.viewport_height),                               // screen up
                81 => self.move_cursor_by(self.cursor.0, self.viewport_height),  // screen down
                c => println!("{c}")
            }

            b'm' => {  // record macro
                self.key_macro.clear();
                let mut c = self.await_input();
                while c != b'm' {
                    self.key_macro.push(c);
                    c = self.await_input();
                }
            },

            b',' => {  // run macro
                self.input.lock().unwrap().append(&mut self.key_macro.clone().into());
            },

            b' ' => {
                self.is_playing = !self.is_playing;
                if self.is_playing {
                    self.play_cursor = -1;
                    self.next_note_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
                }
            },

            b'+' => {
                self.music.notes.insert(self.cursor.1 as usize + 1, [Tone::empty();14]);
                self.move_cursor_by(0, 1);
            },
            b'-' => {
                if self.music.size().1 > 1{
                    let _ = self.music.notes.remove(self.cursor.1 as usize);
                    self.move_cursor_by(0, 0);
                }
            }
            b'i' => {
                if self.cursor.1 > 0 {
                    self.music.notes.swap(self.cursor.1 as usize, self.cursor.1 as usize - 1);
                    self.move_cursor_by(0, -1);
                }
            }
            b'k' => {
                if self.cursor.1 < self.music.size().1 as i32 - 1  {
                    self.music.notes.swap(self.cursor.1 as usize, self.cursor.1 as usize + 1);
                    self.move_cursor_by(0, 1);
                }
            }

            b'#' => { let _ = self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.as_mut().map(|n| n.toggle_sharp()); }
            b'.' => { let _ = self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.as_mut().map(|n| n.short = !n.short); }

            b'q' | 83 => { //  83: rem
                let n = &mut self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note;
                std::mem::swap(n, &mut None)
            },

            b'c' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 1, octave: 4, short: false }).note = 1,
            b'd' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 3, octave: 4, short: false }).note = 3,
            b'e' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 5, octave: 4, short: false }).note = 5,
            b'f' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 6, octave: 4, short: false }).note = 6,
            b'g' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 8, octave: 4, short: false }).note = 8,
            b'a' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 10, octave: 4, short: false }).note = 10,
            b'b' => self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.get_or_insert(Note { note: 12, octave: 4, short: false }).note = 12,

            // numbers 1-9
            c if c >= 48 && c <= 57 => { let _ = self.music.at_mut(self.cursor.0 as usize, self.cursor.1 as usize).note.as_mut().map(|n| n.octave = c - 48); }
            _ => println!("{c}")
        }
    }

    fn handle_playback(&mut self){
        if self.is_playing {
            let millis = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            if millis >= self.next_note_time {
                self.play_cursor = i32::rem_euclid(self.play_cursor + 1, self.music.size().1 as i32);
                self.next_note_time = millis + 1000 / self.music.bps as u128;
                let _ = self.channels.iter_mut().enumerate().map(|(i, channel)| {
                    let mut input = channel.input.lock().unwrap();
                    if let Some(note) = &self.music.at(i, self.play_cursor as usize).note && channel.enabled {
                        input.freq = note.frequency();
                        input.playing = true;
                    } else {
                        input.playing = false;
                    }
                }).collect::<Vec<_>>();
            }
            self.move_cursor_to(0, self.play_cursor + 16);
            self.cursor.0 = -1;
        } else {
            self.channels.iter_mut().map(|channel| channel.input.lock().unwrap().playing = false).collect()
        }
    }

    fn move_cursor_by(&mut self, dx: i32, dy: i32) {
        self.move_cursor_to(self.cursor.0 + dx, self.cursor.1 + dy)
    }

    fn move_cursor_to(&mut self, x: i32, y: i32) {
        self.cursor = (max(min(x, self.music.size().0 as i32 - 1), 0),
                       max(min(y, self.music.size().1 as i32 - 1), 0));
        if self.cursor.1 < self.viewport + 5 {
            self.viewport = i32::max(self.cursor.1 - 5, 0);
        }
        if self.cursor.1 > self.viewport + 27 {
            self.viewport = self.cursor.1 - 27;
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();
        const NAMES: [&'static str; 14] = ["SIN", "SIN", "SIN", "SIN", "SIN", "SIN", "SQR", "SAW", "SQR", "SAW", "TRI", "TRI", "TRI", "TRI"];
        out.push_str("       ");
        for x in 0..self.music.size().0 {
            out.push_str(&format!(" \x1b[1;32m{}\x1b[0m ", NAMES[x]));
        }
        out.push_str("\n");
        let visible = self.viewport_height as f32 / self.music.size().1 as f32;
        let p = self.cursor.1 as f32 / self.music.size().1 as f32;
        let h = (visible * self.viewport_height as f32) as usize; // scroll bar height
        let ph = (p * (self.viewport_height as f32 - h as f32 + 1.0)) as usize; // scroll bar y
        for y in self.viewport as usize..(self.viewport + self.viewport_height) as usize{
            if y as i32 == self.play_cursor {
                out.push_str(&format!(" \x1b[48;5;237m{y:03}\x1b[0m │ "));
            } else {
                out.push_str(&format!(" {y:03} │ "));
            }
            if y >= self.music.size().1 {
                out.push_str(&"      ".repeat(self.music.size().0));
            } else {
                for x in 0..self.music.size().0 {
                    let note = self.music.at(x, y).render();
                    let h = x % 2 == y / self.music.section_height as usize % 2;
                    if (x as i32, y as i32) == self.cursor {
                        out.push_str(&format!("\x1b[48;5;240m {note} \x1b[0m"))
                    } else if y as i32 == self.play_cursor {
                        out.push_str(&format!("{} {note} \x1b[0m", if h { "\x1b[48;5;236m" } else { "\x1b[48;5;237m" }))
                    } else {
                        out.push_str(&format!("{} {note} \x1b[0m", if h { "\x1b[48;5;237m" } else { "\x1b[48;5;238m" }))
                    }
                }
            }
            let absolute_y = y - self.viewport as usize;
            if absolute_y >= ph && absolute_y < ph + h {
                out.push_str(" █");
            } else {
                out.push_str(" ║");
            }
            out.push('\n');
        }
        out
    }

    fn input(&self) -> Option<u8>{
        self.input.lock().unwrap().pop_front()
    }

    fn await_input(&self) -> u8{
        loop {
            if let Some(c) = self.input.lock().unwrap().pop_front() {
                return c
            }
        }
    }

    fn quit(&self) -> ! {
        let ser = self.music.serialize();
        let mut auto_save = File::create("auto_save.syb").expect("error opening file");
        auto_save.write(ser.as_slice()).expect("error writing to file");
        print!("\x1b[?25h");  // show cursor
        exit(0)
    }
}