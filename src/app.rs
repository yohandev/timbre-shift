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
            notes: vec!
            [
                (60, A4), (120, B4),
                (180, C5), (220, D5),
                (280, E5), (340, F5),
                (400, G5), (460, A5),
            ],
        }
    }
}