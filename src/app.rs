use framework::*;

use rodio::*;

use crate::util;

pub struct TimbreShift
{
    /// audio output device
    _stream: OutputStream,
    /// audio output device handle
    audio: OutputStreamHandle,
}

impl App for TimbreShift
{
    fn start(&mut self)
    {
        // open the audio file
        let src = Decoder::new(util::open("res/c_major.ogg"))
            .unwrap()
            .convert_samples::<f32>();

        // play the audio
        self.audio
            .play_raw(src)
            .unwrap();   
    }
}

impl Default for TimbreShift
{
    fn default() -> Self
    {
        let (_stream, audio) = OutputStream::try_default().unwrap();

        Self { _stream, audio }
    }
}