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

        let tempo = |i: f32| { ((10.0 * i.sqrt() + 5.0) * 7.0) as u64 };
        let notes = CHROMATIC_SCALE
            // zelda arpeggios:
            .windows(7)
            .flat_map(|hz| hz
                .iter()
                .enumerate()
                .filter(|(i, _)| match i { 0 | 2 | 4 | 6 => true, _ => false }))
                .map(|(_, hz)| hz)
            .enumerate()
            .map(|(i, hz)| (tempo(i as f32), Beep::note(*hz)))
            // major chords:
            //.windows(7)
            //.enumerate()
            //.map(|(i, hz)| (i as u64 * 30, Beep::chord(&[hz[0], hz[3], hz[6]])))
            .collect::<Vec<_>>();        

        Self
        {
            sound: Speakers::new().unwrap(),
            notes
        }
    }
}