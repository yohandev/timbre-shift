use framework::sound::*;
use framework::*;

use crate::beep::Beep;

pub struct TimbreShift
{
    sound: Speakers,

    /// (temporary) timestamp and note to play
    notes: Vec<(u64, Beep)>,
}

impl App for TimbreShift
{
    fn start(&mut self)
    {
        // // open the audio file
        // let src = Decoder::new(util::open("res/c_major.ogg"))
        //     .unwrap()
        //     .convert_samples::<f32>();

        // // play the audio
        // self.audio
        //     .play_raw(src)
        //     .unwrap();  
        
        // reverse notes to pop() them in order
        self.notes.reverse();
    }

    fn update(&mut self, time: &Time)
    {
        if let Some((tick, note)) = self.notes.last()
        {
            if time.tick() >= *tick
            {
                self.sound.play(note.clone());
                self.notes.pop();
            }
        }
    }
}

impl Default for TimbreShift
{
    fn default() -> Self
    {
        use crate::beep::notes::*;

        Self
        {
            sound: Speakers::new().unwrap(),
            notes: [A3, Bb3, B3, C4, Db4,
                    D4, Eb4, E4, F4, Gb4,
                    G4, Ab4, A4, Bb4, B4,
                    C5, Db5, D5, Eb5, E5,
                    F5, Gb5, G5, Ab5, A5]
                        .iter()
                        .enumerate()
                        .map(|(i, hz)| (i as u64 * 15, Beep::note(*hz)))
                        .collect::<Vec<_>>(),
            
        }
    }
}