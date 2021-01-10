use framework::sound::*;
use framework::math::*;
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

    fn render(&mut self, frame: &mut Frame)
    {
        let mul = self.ogg.sample_rate() / Self::SIZE.w;
        let samples = self.ogg.samples();

        frame.clear(Rgba::black());
        frame.par_iter_pixels_mut().for_each(|(pos, px)|
        {
            fn avg(a: &[f32]) -> f32
            {
                let sum: f32 = a.iter().sum();

                sum / (a.len() as f32)
            }

            let x = pos.x as usize;
            let avg = avg(&samples[x * mul..((x + 1) * mul)]);
            let s = (avg + 1.0) * 0.5;
            let y = (Self::SIZE.h as f32 * s) as i32;
            if pos.y > y
            {
                *px = Rgba::blue();
            }
        });
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