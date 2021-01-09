use framework::sound::*;
use framework::*;

use crate::beep::Beep;

pub struct TimbreShift
{
    // /// audio output device
    // _stream: OutputStream,
    // /// audio output device handle
    // audio: OutputStreamHandle,
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
        self.sound.play(Beep::default());
    }
}

impl Default for TimbreShift
{
    fn default() -> Self
    {
        // let (_stream, audio) = OutputStream::try_default().unwrap();

        // Self { _stream, audio }
        Self { sound: Speakers::new().unwrap() }
    }
}