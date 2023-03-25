use std::time::Duration;

pub use crate::spawn_area::SpawnArea;
pub type Rgb = rgb::RGB<f32>;

#[derive(Debug, Clone)]
pub struct Interval<T> {
    pub start: T,
    pub end: T,
}

impl Interval<f32> {
    #[inline]
    pub fn interpolate(&self, value: f32) -> f32 {
        self.start + ((self.end - self.start) * value)
    }
}

impl Interval<Rgb> {
    #[inline]
    pub fn interpolate(&self, value: f32) -> Rgb {
        self.start + ((self.end - self.start) * value)
    }
}

#[derive(Debug, Clone)]
pub struct Bounds<T> {
    pub min: T,
    pub max: T,
}

#[derive(Debug, Clone)]
pub struct Config {
    // Particle properties
    pub alpha: Interval<f32>,
    pub scale: Interval<f32>,
    // minimum scale multiplier: f32
    pub color: Interval<Rgb>,
    pub speed: Interval<f32>,
    // minimum speed multiplier: f32
    // acceleration: Interval<f32>,
    // maximum speed
    pub start_rotation: Interval<f32>,
    // no particle rotation: bool
    // pub rotation_speed: Interval<f32>,
    pub lifetime: Bounds<Duration>,
    // blend mode
    pub custom_ease: Option<fn(f32) -> f32>,

    // Emitter properties
    pub spawn_frequency: Duration,
    pub emitter_lifetime: Duration,
    pub max_particles: usize,
    pub spawn_area: SpawnArea,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: Interval {
                start: 70.0,
                end: 200.0,
            },

            alpha: Interval {
                start: 1.0,
                end: 1.0,
            },

            scale: Interval {
                start: 0.2,
                end: 0.2,
            },

            start_rotation: Interval {
                start: 80.0,
                end: 70.0,
            },

            custom_ease: None,

            spawn_frequency: Duration::from_secs_f32(0.005),

            color: Interval {
                start: Rgb::new(30.0 / 255.0, 196.0 / 255.0, 247.0 / 255.0),
                end: Rgb::new(168.0 / 255.0, 5.0 / 255.0, 1.0),
            },

            lifetime: Bounds {
                min: Duration::from_secs_f32(0.3),
                max: Duration::from_secs(10),
            },

            emitter_lifetime: Duration::MAX,

            spawn_area: SpawnArea::Rectangle {
                x: -50.0,
                y: -50.0,
                width: 1000.0,
                height: 0.0,
            },

            max_particles: 200000,
        }
    }
}
