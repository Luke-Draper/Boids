use amethyst::ecs::{Component, NullStorage};

pub struct PlayerControl;
impl Component for PlayerControl {
    type Storage = NullStorage<Self>;
}
