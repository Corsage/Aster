use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Deserialize)]
pub struct Npc {
    pub speed: u8,
    pub level: u8,
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
