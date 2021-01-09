use framework::sound::*;

/// beeeeeeeeeeeeeee
#[derive(Default, Debug, Clone, Copy)]
pub struct Beep
{
    /// frequency
    hz: f32,

    /// current phase
    phase: f32,
    /// how much to increment phase by
    phase_inc: f32,

    /// total number of channels
    channels: usize,
    /// counts up down from channels
    channels_iter: usize,
}

impl Track for Beep
{
    type Format = f32;

    fn next_sample(&mut self) -> Option<Self::Format>
    {
        if self.channels_iter == 0
        {
            self.channels_iter = self.channels;
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
        self.channels_iter -= 1;

        Some(self.phase.sin())
    }

    fn tune(&mut self, channels: usize, sample_rate: usize)
    {
        self.phase = 0.0;
        self.phase_inc = self.hz / sample_rate as f32;

        self.channels = channels;
        self.channels_iter = channels;
    }
}

#[allow(dead_code)]
impl Beep
{
    pub fn new(hz: f32) -> Self
    {
        Self { hz, ..Default::default() }    
    }

    /// equivalent of `Beep::new(0.0)` or `Beep::default()`
    pub const fn zero() -> Self
    {
        Self
        {
            hz: 0.0,
            phase: 0.0,
            phase_inc: 0.0,
            channels: 0,
            channels_iter: 0
        }
    }
}

/// beeeeeep ^ beeeeeep ^ beeeeep
pub mod notes
{
    use super::Beep;

    pub const A4: Beep = Beep { hz: 440.000, ..Beep::zero() };
    pub const B4: Beep = Beep { hz: 493.883, ..Beep::zero() };
    pub const C5: Beep = Beep { hz: 523.251, ..Beep::zero() };
    pub const D5: Beep = Beep { hz: 587.330, ..Beep::zero() };
    pub const E5: Beep = Beep { hz: 659.255, ..Beep::zero() };
    pub const F5: Beep = Beep { hz: 698.456, ..Beep::zero() };
    pub const G5: Beep = Beep { hz: 783.991, ..Beep::zero() };
    pub const A5: Beep = Beep { hz: 880.000, ..Beep::zero() };
}