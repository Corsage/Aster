use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Npc {
    pub name: String,
    pub speed: u8,
    pub level: u8,
    pub job: Job,
    pub action: Action,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Job {
    #[default]
    Normal,
    Enemy,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Action {
    #[default]
    Idle,
    Talk,
}
