use std::mem::MaybeUninit;

pub(crate) struct Music {
    pub(crate) notes: Vec<[Tone;14]>
}

impl Music {
    pub(crate) fn at(&self, x: usize, y: usize) -> &Tone{
        &self.notes.get(y).unwrap()[x]
    }

    pub(crate) fn at_mut(&mut self, x: usize, y: usize) -> &mut Tone{
        &mut self.notes.get_mut(y).unwrap()[x]
    }

    pub(crate) fn size(&self) -> (usize, usize) {
        (14, self.notes.len())
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        let file_version = 0;
        let width = 14;
        let mut ser = vec![file_version, width];
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                ser.append(&mut self.at(x, y).serialize())
            }
        }
        ser
    }

    pub(crate) fn deserialize(bytes: &mut dyn Iterator<Item=u8>) -> Self {
        let mut bytes = bytes.peekable();
        let file_version = bytes.next().unwrap();
        let width = bytes.next().unwrap();
        assert_eq!(file_version, 0, "invalid file version");
        assert_eq!(width, 14, "invalid width");
        let mut notes = vec![];
        while bytes.peek().is_some() {
            let mut row: [MaybeUninit<Tone>; 14] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            for i in 0..width {
                row[i as usize] = MaybeUninit::new(Tone::deserialize(&mut bytes));
            }
            notes.push(unsafe { std::mem::transmute::<_, [Tone; 14]>(row) })
        }
        Self {
            notes,
        }
    }
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

    pub(crate) fn note(note: u8, octave: u8) -> Self{
        Tone {
            note: Some(Note {
                note,
                octave,
            })
        }
    }

    pub(crate) fn render(&self) -> String{
        if let Some(n) = &self.note {
            n.render()
        } else {
            String::from("\x1b[38;5;245m---")
        }
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        self.note.as_ref().map(|note| vec![note.note, note.octave]).unwrap_or(vec![0])
    }

    pub(crate) fn deserialize(bytes: &mut dyn Iterator<Item=u8>) -> Self {
        let n = bytes.next().unwrap();
        if n != 0 {
            let o = bytes.next().unwrap();
            Tone::note(n, o)
        } else {
            Tone::empty()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Note {
    pub(crate) note: u8,
    pub(crate) octave: u8
}

impl Note {
    pub(crate) fn render(&self) -> String{
        format!("{}{}", match self.note {
            1 => "\x1b[38;5;196mC-",
            2 => "\x1b[38;5;202mC#",
            3 => "\x1b[38;5;208mD-",
            4 => "\x1b[38;5;220mD#",
            5 => "\x1b[38;5;190mE-",
            6 => "\x1b[38;5;118mF-",
            7 => "\x1b[38;5;48mF#",
            8 => "\x1b[38;5;51mG-",
            9 => "\x1b[38;5;39mG#",
            10 => "\x1b[38;5;21mA-",
            11 => "\x1b[38;5;129mA#",
            12 => "\x1b[38;5;201mB-",
            _ => panic!("invalid note {}", self.note)
        }, self.octave)
    }
    pub(crate) fn frequency(&self) -> f32 {
        let n = self.note + 12 * self.octave;
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