use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct PlayerControl;
impl Component for PlayerControl {
    type Storage = NullStorage<Self>;
}
