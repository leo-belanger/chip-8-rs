use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    Sdl,
};
use std::error::Error;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Speaker {
    audio_device: AudioDevice<SquareWave>,
}

impl Speaker {
    pub fn new(sdl_context: &Sdl) -> Result<Speaker, Box<dyn Error>> {
        let audio_subsystem = sdl_context.audio()?;

        let audio_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| SquareWave {
            phase_inc: 750.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.05,
        })?;

        Ok(Speaker { audio_device })
    }

    pub fn start_beep(&self) {
        self.audio_device.resume();
    }

    pub fn stop_beep(&self) {
        self.audio_device.pause();
    }
}
