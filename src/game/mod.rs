use bevy::prelude::*;

use self::{
    actions::ActionsPlugin, manager::ManagerPlugin, map::MapPlugin, npc::NpcPlugin,
    player::PlayerPlugin,
};

pub mod actions;
pub mod components;
mod manager;
pub mod map;
pub mod npc;
pub mod pieces;
pub mod player;

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
    Paused,
    Running,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(MapPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ManagerPlugin)
            .add_plugin(ActionsPlugin);
    }
}
