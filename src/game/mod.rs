use bevy::prelude::*;

use self::{map::MapPlugin, npc::NpcPlugin};

pub mod components;
pub mod map;
pub mod npc;

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Paused,
    Running,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(MapPlugin)
            .add_plugin(NpcPlugin);
    }
}
