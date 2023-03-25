use crate::config::Config;
use crate::particle_state::ParticleState;
type Rgb = rgb::RGB<f32>;

pub struct Particles<'a, I>
where
    I: Iterator<Item = &'a ParticleState>,
{
    pub(crate) states: I,
    pub(crate) config: &'a Config,
}

impl<'a, I> Iterator for Particles<'a, I>
where
    I: Iterator<Item = &'a ParticleState>,
{
    type Item = Particle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.states.next()?;
        Some(Particle {
            state,
            config: self.config,
        })
    }
}

#[derive(Debug)]
pub struct Particle<'a> {
    state: &'a ParticleState,
    config: &'a Config,
}

impl<'a> Particle<'a> {
    pub fn color(&self) -> Rgb {
        self.config.color.interpolate(self.state.progress)
    }

    pub fn alpha(&self) -> f32 {
        self.config.alpha.interpolate(self.state.progress)
    }

    pub fn coords(&self) -> (f32, f32) {
        let ParticleState { x, y, .. } = *self.state;
        (x, y)
    }

    pub fn scale(&self) -> f32 {
        self.config.scale.interpolate(self.state.progress)
    }

    pub fn image_index(&self) -> usize {
        self.state.image_index
    }
}
