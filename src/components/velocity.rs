use amethyst::{
    core::math::Vector3,
    ecs::{Component, VecStorage},
};

pub struct Velocity {
    pub velocity: f32,
    pub direction: Vector3<f32>, // linear, angular velocity is not needed.
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
