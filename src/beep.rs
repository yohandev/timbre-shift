#![allow(dead_code)]

use framework::sound::*;

/// beeeeeeeeeeeeeee
#[derive(Default, Debug, Clone)]
pub struct Beep
{
    /// (frequency, clock, cached sine multiplier)
    waves: Box<[(f32, f32, f32)]>,
    /// volume multiplier of an individual note
    vol: f32,

    /// total number of channels
    channels: usize,
    /// counts up down from channels
    channels_iter: usize,
    /// output sample rate cached
    sample_rate: f32,
}

impl Track for Beep
{
    type Format = f32;

    fn next_sample(&mut self) -> Option<Self::Format>
    {
        // increment once for every channel
        // TODO: make API better for channels
        if self.channels_iter == 0
        {
            self.channels_iter = self.channels;

            for (_, clock, _) in self.waves.iter_mut()
            {
                *clock = (*clock + 1.0) % self.sample_rate;
            }
        }
        self.channels_iter -= 1;

        // sum waves
        Some(self.waves
            .iter()
            .map(|(_, c, m)| (c * m).sin() * self.vol)
            .sum())
    }

    fn tune(&mut self, channels: usize, sample_rate: usize)
    {
        use std::f32::consts::PI;

        // conf
        self.channels = channels;
        self.channels_iter = channels;
        self.sample_rate = sample_rate as f32;

        // build waves
        for (hz, clock, mult) in self.waves.iter_mut()
        {
            *clock = 0.0;
            *mult = *hz * 2.0 * PI / self.sample_rate;
        }
    }
}

impl Beep
{
    /// create a beep from a single note's frequency
    pub fn note(hz: f32) -> Self
    {
        Self::chord(&[hz])
    }

    /// create a beep form a chord's note frequencies
    pub fn chord(hz: &[f32]) -> Self
    {
        let waves = hz
            .iter()
            .map(|f| (*f, 0.0, 0.0))
            .collect::<Box<[_]>>();
        let vol = waves.len() as f32;

        Self
        {
            waves,
            vol,
            channels: 0,
            channels_iter: 0,
            sample_rate: 0.0,
        }
    }
}

/// beeeeeep ^ beeeeeep ^ beeeeep
#[allow(non_upper_case_globals)]
pub mod notes
{
    pub const CHROMATIC_SCALE: [f32; 25] = [A3, Bb3, B3, C4, Db4,
                                            D4, Eb4, E4, F4, Gb4,
                                            G4, Ab4, A4, Bb4, B4,
                                            C5, Db5, D5, Eb5, E5,
                                            F5, Gb5, G5, Ab5, A5];

    pub const A3:  f32 = 220.00;
    pub const Bb3: f32 = 233.08;
    pub const B3:  f32 = 246.94;
    pub const C4:  f32 = 261.63;
    pub const Db4: f32 = 277.18;
    pub const D4:  f32 = 293.66;
    pub const Eb4: f32 = 311.13;
    pub const E4:  f32 = 329.63;
    pub const F4:  f32 = 349.23;
    pub const Gb4: f32 = 369.99;
    pub const G4:  f32 = 392.00;
    pub const Ab4: f32 = 415.30;
    pub const A4:  f32 = 440.00;
    pub const Bb4: f32 = 466.16;
    pub const B4:  f32 = 493.88;
    pub const C5:  f32 = 523.25;
    pub const Db5: f32 = 554.37;
    pub const D5:  f32 = 587.33;
    pub const Eb5: f32 = 622.25;
    pub const E5:  f32 = 659.25;
    pub const F5:  f32 = 698.46;
    pub const Gb5: f32 = 739.99;
    pub const G5:  f32 = 783.99;
    pub const Ab5: f32 = 830.61;
    pub const A5:  f32 = 880.00;
}