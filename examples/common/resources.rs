use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct ParticleConfiguration {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub damping: f32,
}

#[derive(Debug, Clone, Resource)]
pub struct SpawnConfiguration {
    pub particles: Vec<ParticleConfiguration>,
}

impl Default for SpawnConfiguration {
    fn default() -> Self {
        Self { particles: vec![] }
    }
}

impl SpawnConfiguration {
    pub fn new(particles: Vec<ParticleConfiguration>) -> Self {
        Self { particles }
    }
}
