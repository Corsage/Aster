use bevy::prelude::*;
use game::GamePlugin;
use graphics::{GraphicsPlugin, TILE_SIZE};
use menu::MenuPlugin;
use splash::SplashPlugin;
use ui::UiPlugin;
use utils::UtilsPlugin;

pub mod game;
mod graphics;
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
        // Utils.
        .add_plugin(UtilsPlugin)
        // UI.
        .add_plugin(UiPlugin)
        // Splash screen.
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GraphicsPlugin)
        .add_startup_system(setup)
        .run();
}

/// Menu Camera.
/// This spawns the camera specifically for the menu.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4. * TILE_SIZE,
        4. * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
