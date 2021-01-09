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

        let notes = CHROMATIC_SCALE
            .windows(5)
            .enumerate()
            .map(|(i, hz)| (i as u64 * 30, Beep::chord(&[hz[0], hz[2], hz[4]])))
            .collect::<Vec<_>>();

        Self
        {
            sound: Speakers::new().unwrap(),
            notes 
            
        }
    }
}