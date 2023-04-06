use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;
use splash::SplashPlugin;
use ui::UiPlugin;

mod game;
mod menu;
mod splash;
mod ui;
pub mod utils;

/// AppState
/// Contains the various states the application can be in.
/// This is different from the GameState.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
    Error,
}

fn main() {
    App::new()
        // Init default Bevy plugins.
        .add_plugins(DefaultPlugins)
        // Track application state.
        .add_state::<AppState>()
        // Splash screen.
        .add_plugin(SplashPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}
