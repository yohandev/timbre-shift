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

#[allow(dead_code)]
impl Beep
{
    pub const A4: Self = Self { phase: 0.0, phase_inc: 440.0 / 44_100.0 };
    pub const B4: Self = Self { phase: 0.0, phase_inc: 493.883 / 44_100.0 };
    pub const C5: Self = Self { phase: 0.0, phase_inc: 523.251 / 44_100.0 };
    pub const D5: Self = Self { phase: 0.0, phase_inc: 587.330 / 44_100.0 };
    pub const E5: Self = Self { phase: 0.0, phase_inc: 659.255 / 44_100.0 };
    pub const F5: Self = Self { phase: 0.0, phase_inc: 698.456 / 44_100.0 };
    pub const G5: Self = Self { phase: 0.0, phase_inc: 783.991 / 44_100.0 };
    pub const A5: Self = Self { phase: 0.0, phase_inc: 880.0 / 44_100.0 };

    pub fn new(hz: f32) -> Self
    {
        Self { phase: 0.0, phase_inc: hz / 44_100.0 }    
    }
}