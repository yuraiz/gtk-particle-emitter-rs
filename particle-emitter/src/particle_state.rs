use crate::config::{Bounds, Config};

use rand::random;
use std::time::Duration;

#[derive(Debug)]
pub struct ParticleState {
    pub x: f32,
    pub y: f32,

    pub start_rotation: f32,
    pub image_index: usize,

    creation_time: Duration,
    lifetime: Duration,

    pub progress: f32,
}

impl ParticleState {
    pub fn new(creation_time: Duration, config: &Config) -> Self {
        let lifetime = {
            let Bounds { min, max } = config.lifetime;
            ((max - min).mul_f32(random())) + min
        };

        let (x, y) = config.spawn_area.gen_coord();

        Self {
            x,
            y,
            lifetime,
            image_index: random(),
            creation_time,
            start_rotation: config.start_rotation.interpolate(random()),
            progress: 0.0,
        }
    }

    pub fn refresh_progress(&mut self, elapsed: Duration, custom_ease: Option<fn(f32) -> f32>) {
        let progress = (elapsed - self.creation_time).as_secs_f32() / self.lifetime.as_secs_f32();

        if let Some(ease) = custom_ease {
            self.progress = ease(progress);
        } else {
            self.progress = progress;
        }
    }

    pub fn kill_time(&self) -> Duration {
        self.creation_time + self.lifetime
    }
}
