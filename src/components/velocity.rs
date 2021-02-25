use amethyst::{
    core::math::Vector3,
    ecs::{Component, VecStorage},
};

pub struct Velocity {
    velocity: f32
    direction: Vector3<f32>; // linear, angular velocity is not needed.
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
