use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct MidiPlayer {
    conn_out: MidiOutputConnection
}

impl MidiPlayer {
    pub fn new() -> MidiPlayer {
        MidiPlayer {
            conn_out: MidiPlayer::get_conn_out()
        }
    }

    pub fn play_note(&mut self, note: u8) {
        let duration = 4;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        match self.conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]) {
            Ok(()) => (),
            Err(the_error) => println!("{}", the_error.to_string())
        }
        sleep(Duration::from_millis(duration * 150));
        self.conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]).unwrap();
    }

    fn get_conn_out() -> MidiOutputConnection {
        let midi_out = match MidiOutput::new("My Test Output") {
            Ok(midi_out) => midi_out,
            Err(_the_error) => panic!("Failed to create new midi output")
        };
    
        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_port: &MidiOutputPort = match out_ports.len() {
            0 => panic!("no output port found"),
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
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                out_ports.get(input.trim().parse::<usize>().unwrap())
                        .ok_or("invalid output port selected").unwrap()
            }
        };

        println!("\nOpening connection");
        let conn_out = midi_out.connect(out_port, "doesn't seem to matter").unwrap();
        println!("Connection open. Listen!");
        return conn_out;
    }
}