use bevy::prelude::*;
use serde::Deserialize;
#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: u8,
}

#[derive(Component, Debug, Deserialize)]
pub struct Character {
    pub name: String,
    pub speed: u8,
    pub level: u8,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize)]
pub enum Job {
    #[default]
    Normal,
    Enemy,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize)]
pub enum Action {
    #[default]
    Idle,
    Talk,
}
