#![allow(dead_code)]

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

/// ~ beeeeeeeeeeeee ~
pub struct HarmoniousBeep
{
    /// all the beeps within this beep
    beeps: Box<[Beep]>
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

impl Track for HarmoniousBeep
{
    type Format = f32;

    fn next_sample(&mut self) -> Option<Self::Format>
    {
        let len = self.beeps.len() as f32;
        let sum = self.beeps
            .iter_mut()
            .map(|i| i.next_sample().unwrap() / len)
            .sum();
        
        Some(sum)
    }

    fn tune(&mut self, channels: usize, sample_rate: usize)
    {
        self.beeps
            .iter_mut()
            .for_each(|i| i.tune(channels, sample_rate));
    }
}

impl HarmoniousBeep
{
    pub const fn new(beeps: Box<[Beep]>) -> Self
    {
        Self { beeps }
    }
}

/// beeeeeep ^ beeeeeep ^ beeeeep
#[allow(non_upper_case_globals)]
pub mod notes
{
    use super::{ Beep, HarmoniousBeep };

    pub const A3:  Beep = Beep { hz: 220.00, ..Beep::zero() };
    pub const Bb3: Beep = Beep { hz: 233.08, ..Beep::zero() };
    pub const B3:  Beep = Beep { hz: 246.94, ..Beep::zero() };
    pub const C4:  Beep = Beep { hz: 261.63, ..Beep::zero() };
    pub const Db4: Beep = Beep { hz: 277.18, ..Beep::zero() };
    pub const D4:  Beep = Beep { hz: 293.66, ..Beep::zero() };
    pub const Eb4: Beep = Beep { hz: 311.13, ..Beep::zero() };
    pub const E4:  Beep = Beep { hz: 329.63, ..Beep::zero() };
    pub const F4:  Beep = Beep { hz: 349.23, ..Beep::zero() };
    pub const Gb4: Beep = Beep { hz: 369.99, ..Beep::zero() };
    pub const G4:  Beep = Beep { hz: 392.00, ..Beep::zero() };
    pub const Ab4: Beep = Beep { hz: 415.30, ..Beep::zero() };
    pub const A4:  Beep = Beep { hz: 440.00, ..Beep::zero() };
    pub const Bb4: Beep = Beep { hz: 466.16, ..Beep::zero() };
    pub const B4:  Beep = Beep { hz: 493.88, ..Beep::zero() };
    pub const C5:  Beep = Beep { hz: 523.25, ..Beep::zero() };
    pub const Db5: Beep = Beep { hz: 554.37, ..Beep::zero() };
    pub const D5:  Beep = Beep { hz: 587.33, ..Beep::zero() };
    pub const Eb5: Beep = Beep { hz: 622.25, ..Beep::zero() };
    pub const E5:  Beep = Beep { hz: 659.25, ..Beep::zero() };
    pub const F5:  Beep = Beep { hz: 698.46, ..Beep::zero() };
    pub const Gb5: Beep = Beep { hz: 739.99, ..Beep::zero() };
    pub const G5:  Beep = Beep { hz: 783.99, ..Beep::zero() };
    pub const Ab5: Beep = Beep { hz: 830.61, ..Beep::zero() };
    pub const A5:  Beep = Beep { hz: 880.00, ..Beep::zero() };

    pub fn a_major() -> HarmoniousBeep
    {
        HarmoniousBeep::new(Box::new([A3, Db4, E4]))
    }
}