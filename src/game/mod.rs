use bevy::prelude::*;

use self::npc::NpcPlugin;

pub mod components;
pub mod npc;

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugin(NpcPlugin);
    }
}
