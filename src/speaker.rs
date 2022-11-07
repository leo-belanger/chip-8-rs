use sdl2::{audio::AudioSpecDesired, AudioSubsystem, Sdl};

pub struct Speaker {
    audio_spec: AudioSpecDesired,
    audio_subsystem: AudioSubsystem,
}

impl Speaker {
    pub fn new(sdl_context: &Sdl) -> Speaker {
        let audio_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };
        let audio_subsystem = sdl_context.audio().unwrap();

        Speaker {
            audio_spec,
            audio_subsystem,
        }
    }

    pub fn start_beep(&self) {}

    pub fn stop_beep(&self) {}
}
