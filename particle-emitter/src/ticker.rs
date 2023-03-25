use crate::config::Config;
use crate::particle::Particles;
use crate::particle_state::ParticleState;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Ticker {
    creation_time: Instant,
    last_tick_time: Duration,
    spawned: u128,
    pub config: Config,
    pub particle_states: VecDeque<ParticleState>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            spawned: 0,
            last_tick_time: Duration::ZERO,
            creation_time: Instant::now(),
            config: Default::default(),
            particle_states: VecDeque::new(),
        }
    }
}

impl Ticker {
    pub fn particles(&self) -> Particles<impl Iterator<Item = &ParticleState>> {
        Particles {
            states: self.particle_states.iter(),
            config: &self.config,
        }
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        let Self {
            spawned,
            creation_time,
            config,
            particle_states: particles,
            last_tick_time,
        } = self;

        let elapsed = creation_time.elapsed();
        let delta = elapsed - *last_tick_time;
        *last_tick_time = elapsed;

        if elapsed < config.emitter_lifetime {
            let count_to_spawn = elapsed.as_nanos() / config.spawn_frequency.as_nanos();

            for _ in 0..count_to_spawn.saturating_sub(*spawned) {
                particles.push_front(ParticleState::new(elapsed, config));
            }

            *spawned = count_to_spawn;
        }

        particles
            .iter_mut()
            .for_each(|particle| Self::update_particle(particle, elapsed, delta, config));

        particles.retain(|particle| particle.kill_time() > elapsed);

        if particles.len() > config.max_particles {
            particles
                .make_contiguous()
                .sort_unstable_by_key(|p| p.kill_time());
            particles.truncate(config.max_particles);
        }
    }

    fn update_particle(
        particle: &mut ParticleState,
        elapsed: Duration,
        delta: Duration,
        config: &Config,
    ) {
        particle.refresh_progress(elapsed, config.custom_ease);

        let progress = particle.progress;

        let speed = config.speed.interpolate(progress) * delta.as_secs_f32();

        let rotation = particle.start_rotation;

        let (scale_y, scale_x) = (rotation * (std::f32::consts::PI / 180.0)).sin_cos();

        particle.x += speed * scale_x;
        particle.y += speed * scale_y;
    }
}
