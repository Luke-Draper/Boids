use super::super::{
    components::{boid::Boid, player_control::PlayerControl, velocity::Velocity},
    configurations::boid_species::BoidSpeciesConfiguration,
};
use amethyst::{
    core::{timing::Time, math::Vector3, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct BoidSystem;

impl<'s> System<'s> for BoidSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Boid>,
        WriteStorage<'s, Velocity>,
        Read<'s, Time>,
        Read<'s, BoidSpeciesConfiguration>,
    );

    fn run(&mut self, (mut transforms, mut boids, mut velocities, time, config): Self::SystemData) {
        for (transform, boid, velocity) in (&mut transforms, &mut boids, &mut velocities).join() {
            transform.append_translation(Vector3::new(
                velocity.velocity * time.delta_seconds(),
                0.0,
                0.0,
            ));
        }
    }
}
