use framework::sound::*;
use framework::*;

pub struct TimbreShift
{
    sound: Speakers,
    ogg: SoundFile<f32>,
}

impl App for TimbreShift
{
    fn start(&mut self)
    {
        self.sound.play(self.ogg.track());
    }
}

impl Default for TimbreShift
{
    fn default() -> Self
    {
        // create speakers
        let sound = Speakers::new().unwrap();

        // open the audio file
        let ogg = SoundFile::open("res/c_major.ogg").unwrap();

        Self { sound, ogg }
    }
}