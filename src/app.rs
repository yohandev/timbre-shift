use framework::sound::*;
use framework::*;

use crate::beep::{ HarmoniousBeep };

pub struct TimbreShift
{
    sound: Speakers,

    /// (temporary) timestamp and note to play
    notes: Vec<(u64, HarmoniousBeep)>,
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
            notes: [a_major(),]
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (i as u64 * 60, n.clone()))
                        .collect::<Vec<_>>(),
            
        }
    }
}