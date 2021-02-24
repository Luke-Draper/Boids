use amethyst::ecs::{Component, VecStorage};

pub enum WingFlapStage {
    Up,
    Middle,
    Down,
    Ground
}

pub struct Wing {
    flap_stage: WingFlapStage,
    flap_time: f64 // Time.absolute_time_seconds() // time since game start adjusted for game speed
};
impl Component for Wing {
    type Storage = VecStorage<Self>;
}
