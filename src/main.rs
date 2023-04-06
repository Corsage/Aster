use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;
use ui::UiPlugin;

mod game;
mod menu;
mod ui;

/// AppState
/// Contains the various states the application can be in.
/// This is different from the GameState.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(UiPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}
