use chess::cell::{self, Cell};

use crate::chess;

extern crate midir;

use midir::{MidiOutput, MidiOutputPort};
use std::{error::Error, vec};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct Pitch {
    base_midi: i32,
    adjustment: i32
}

impl Pitch {
    fn new(file: char) -> Pitch {
        Pitch {
            base_midi: Pitch::file_to_midi(file),
            adjustment: 0
        }
    }
    fn new_with_cell(cell: &Cell) -> Pitch {
        Pitch {
            base_midi: Pitch::file_to_midi(cell.file),
            adjustment: cell.row - 1
        }
    }

    // transposition is x and y on a chess board
    fn new_with_cell_diff(&self, cell_diff: (i32, i32)) -> Pitch {
        let (x, y) = cell_diff;
         Pitch {
            base_midi: self.base_midi + (y * 2),
            adjustment: self.adjustment + x
        }
    }

    fn file_to_midi(file: char) -> i32 {
        match file {
            'a' => 57,
            'b' => 59,
            'c' => 60,
            'd' => 62,
            'e' => 64,
            'f' => 65,
            'g' => 67,
            'h' => 69,
            _ => panic!("unexpected file {}", file)
        }
    }

    pub fn as_midi(&self) -> i32 {
        self.base_midi + self.adjustment
    }

    fn base_midi_with_offset(base_midi: char, offset: i32) -> char {
        let char_as_digit = base_midi as i32;
        let char_with_offset = char_as_digit + offset;
        let new_name = if char_with_offset > 0 {char_with_offset as u8 as char} else {panic!("error converting number to char")};

        if ! ['a', 'b', 'c', 'd', 'e', 'f', 'g'].contains(&new_name) {
            panic!("error converting number to char")
        }

        new_name
    }

    pub fn get_pitches_from_cell_history(cell_history: &Vec<Cell>) -> Vec<Pitch> {
        let base_cell = cell_history[0];
        let base_pitch = Pitch::new_with_cell(&base_cell);
        let mut pitches = vec![base_pitch];

        for cell in cell_history[1..].iter() {
            let cell_diff = base_cell.get_cell_diff(cell);
            let new_pitch = pitches[0].new_with_cell_diff(cell_diff);
            pitches.push(new_pitch);
        }
        pitches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pitch_with_cell() {
        let pitch = Pitch::new_with_cell(&Cell::new("a1"));
        assert_eq!(pitch.base_midi, 57);
        assert_eq!(pitch.adjustment, 0);
    }

    #[test]
    fn test_new_pitch_with_cell_diff() {
        let pitch = Pitch::new('a');
        let new_pitch = pitch.new_with_cell_diff((0, 1));
        assert_eq!(new_pitch.base_midi, 59);
        assert_eq!(new_pitch.adjustment, 0);

        let new_pitch = pitch.new_with_cell_diff((1, 0));
        assert_eq!(new_pitch.base_midi, 57);
        assert_eq!(new_pitch.adjustment, 1);
    }

    // #[test]
    fn test_midi() -> Result<(), Box<dyn Error>> {
        let midi_out = MidiOutput::new("My Test Output")?;
    
        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_port: &MidiOutputPort = match out_ports.len() {
            0 => return Err("no output port found".into()),
            1 => {
                println!("Choosing the only available output port: {}", midi_out.port_name(&out_ports[0]).unwrap());
                &out_ports[0]
            },
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports.get(input.trim().parse::<usize>()?)
                        .ok_or("invalid output port selected")?
            }
        };

        println!("\nOpening connection");
        let mut conn_out = midi_out.connect(out_port, "doesn't seem to matter")?;
        println!("Connection open. Listen!");
        {
            // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
            let mut play_note = |note: u8, duration: u64| {
                const NOTE_ON_MSG: u8 = 0x90;
                const NOTE_OFF_MSG: u8 = 0x80;
                const VELOCITY: u8 = 0x64;
                // We're ignoring errors in here
                match conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]) {
                    Ok(()) => println!("worked"),
                    Err(the_error) => println!("{}", the_error.to_string())
                }
                sleep(Duration::from_millis(duration * 150));
                let ret = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
            };

            sleep(Duration::from_millis(4 * 150));
            
            play_note(66, 4);
            play_note(65, 3);
            play_note(63, 1);
            play_note(61, 6);
            play_note(59, 2);
            play_note(58, 4);
            play_note(56, 4);
            play_note(54, 4);
        }
        sleep(Duration::from_millis(150));
        println!("\nClosing connection");
        // This is optional, the connection would automatically be closed as soon as it goes out of scope
        conn_out.close();
        println!("Connection closed");
        Ok(())
    }
}
