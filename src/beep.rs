use framework::sound::*;

/// beeeeeeeeeeeeeee
pub struct Beep
{
    phase: f32,
    phase_inc: f32,
}

impl SampleIterator for Beep
{
    type Format = f32;

    fn next_sample(&mut self) -> Option<Self::Format>
    {
        self.phase = (self.phase + self.phase_inc) % 1.0;
        Some(self.phase)
    }
}

impl Default for Beep
{
    fn default() -> Self
    {
        Self { phase: 0.0, phase_inc: 440.0 / 44_100.0 }    
    }
}