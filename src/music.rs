use std::process::exit;
use crate::upgrade::upgrade;

pub(crate) struct Music {
    pub(crate) bps: u8,
    pub(crate) section_height: u8,
    pub(crate) notes: Vec<Vec<Tone>>,
    pub(crate) ic: Vec<InstrChannel>
}

impl Music {
    pub(crate) fn at(&self, x: usize, y: usize) -> &Tone{
        &self.notes.get(y).unwrap()[x]
    }

    pub(crate) fn at_mut(&mut self, x: usize, y: usize) -> &mut Tone{
        &mut self.notes.get_mut(y).unwrap()[x]
    }

    pub(crate) fn size(&self) -> (usize, usize) {
        (self.ic.len(), self.notes.len())
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        let file_version = 2;
        let mut ser = vec![file_version, self.size().0 as u8, self.bps, self.section_height];
        for i in &self.ic {
            ser.append(&mut i.id.to_be_bytes().into());
            ser.push(i.volume);
            ser.push(i.enabled as u8)
        }
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                ser.append(&mut self.at(x, y).serialize())
            }
        }
        ser
    }

    pub(crate) fn deserialize(bytes: Vec<u8>) -> Self {
        let bytes = upgrade(bytes);
        let mut bytes = bytes.into_iter().peekable();
        let file_version = bytes.next().unwrap();
        let width = bytes.next().unwrap();
        let bps = bytes.next().unwrap();
        let section_height = bytes.next().unwrap();
        assert_eq!(file_version, 2, "invalid file version");
        let mut notes = vec![];
        let mut ic = vec![];
        for _ in 0..width {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();
            let id = u16::from_be_bytes([a, b]);
            let volume = bytes.next().unwrap();
            let enabled = bytes.next().unwrap() > 0;
            ic.push(InstrChannel { id, volume, enabled })
        }
        while bytes.peek().is_some() {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Tone::deserialize(&mut bytes));
            }
            notes.push(row)
        }
        Self {
            bps,
            section_height,
            notes,
            ic
        }
    }
}

pub(crate) struct InstrChannel {
    pub(crate) id: u16,
    pub(crate) volume: u8,
    pub(crate) enabled: bool
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Tone {
    pub(crate) note: Option<Note>,
}

impl Tone {
    pub(crate) fn empty() -> Self{
        Tone {
            note: None
        }
    }

    pub(crate) fn note(note: u8, octave: u8, short: bool) -> Self{
        Tone {
            note: Some(Note {
                short,
                note,
                octave,
            })
        }
    }

    pub(crate) fn render(&self) -> String{
        if let Some(n) = &self.note {
            n.render()
        } else {
            String::from("\x1b[38;5;245m----")
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        self.note.as_ref().map(|note| vec![note.note, note.octave | if note.short { 0b1000_0000 } else {0}]).unwrap_or(vec![0])
    }

    pub(crate) fn deserialize(bytes: &mut dyn Iterator<Item=u8>) -> Self {
        let note = bytes.next().unwrap();
        if note != 0 {
            let d = bytes.next().unwrap();
            let octave = d & 0b0111_1111;
            let short = d & 0b1000_0000 > 0;
            Tone::note(note, octave, short)
        } else {
            Tone::empty()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Note {
    pub(crate) note: u8,
    pub(crate) octave: u8,
    pub(crate) short: bool
}

impl Note {
    pub(crate) fn render(&self) -> String{
        format!("{}{}{}{}", match self.note {
            1 => "\x1b[38;5;196mC-",
            2 => "\x1b[38;5;202mC#",
            3 => "\x1b[38;5;208mD-",
            4 => "\x1b[38;5;220mD#",
            5 => "\x1b[38;5;190mE-",
            6 => "\x1b[38;5;118mF-",
            7 => "\x1b[38;5;48mF#",
            8 => "\x1b[38;5;51mG-",
            9 => "\x1b[38;5;39mG#",
            10 => "\x1b[38;5;27mA-",
            11 => "\x1b[38;5;129mA#",
            12 => "\x1b[38;5;201mB-",
            _ => panic!("invalid note {}", self.note)
        }, match self.octave {
            0 => "\x1b[38;5;105m",
            1 => "\x1b[38;5;33m",
            2 => "\x1b[38;5;51m",
            3 => "\x1b[38;5;123m",
            4 => "\x1b[38;5;118m",
            5 => "\x1b[38;5;190m",
            6 => "\x1b[38;5;202m",
            7 => "\x1b[38;5;196m",
            8 => "\x1b[38;5;200m",
            9 => "\x1b[38;5;129m",
            _ => panic!("invalid octave {}", self.octave)
        }, self.octave, if self.short { "\x1b[38;5;251m!" } else { "\x1b[38;5;245m." } )
    }
    pub(crate) fn frequency(&self) -> f32 {
        // conversion from own system to midi
        let n = (self.note + 3) % 12 + 12 * if self.note >= 10 { self.octave+1 } else { self.octave };
        f32::powf(2.0, (n as f32 - 49.0) / 12.0) * 440.0
    }

    pub(crate) fn is_sharp(&self) -> bool {
        match self.note {
            1 => false,
            2 => true,
            3 => false,
            4 => true,
            5 => false,
            6 => false,
            7 => true,
            8 => false,
            9 => true,
            10 => false,
            11 => true,
            12 =>false,
            _ => panic!("invalid note {}", self.note)
        }
    }

    pub(crate) fn toggle_sharp(&mut self) {
        self.note = match self.note {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            5 => 5,
            6 => 7,
            7 => 6,
            8 => 9,
            9 => 8,
            10 => 11,
            11 => 10,
            12 => 12,
            _ => panic!("invalid note {}", self.note)
        };
    }
}
