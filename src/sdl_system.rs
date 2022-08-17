use sdl2::audio::AudioDevice;
use sdl2::video::Window;
use sdl2::{AudioSubsystem, EventPump, Sdl, VideoSubsystem};

pub struct SDLSystem {
    pub ctx: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub window: Window,
}

impl SDLSystem {
    pub fn new() -> SDLSystem {
        let ctx = sdl2::init().unwrap();
        let video_subsystem = ctx.video().unwrap();
        let window = video_subsystem
            .window("Terrible NES", 1024 + 256, 480)
            .position_centered()
            .build()
            .unwrap();
        SDLSystem {
            ctx,
            video_subsystem,
            window,
        }
    }
}

impl Default for SDLSystem {
    fn default() -> SDLSystem {
        SDLSystem::new()
    }
}
