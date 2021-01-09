use framework::sound::*;
use framework::*;

use crate::beep::Beep;

pub struct TimbreShift
{
    sound: Speakers
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
        
        // temporary... wait for app to be ready
        // TODO change start() call in framework-rs
        std::thread::sleep_ms(1000);

        self.sound.play(Beep::A4);
    }
}

impl Default for TimbreShift
{
    fn default() -> Self
    {
        Self { sound: Speakers::new().unwrap() }
    }
}