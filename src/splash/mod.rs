use bevy::{asset::LoadState, prelude::*};

use crate::{
    utils::text_asset_loader::{DataAssets, TomlAsset},
    AppState,
};

use self::systems::layout::*;

mod components;
mod styles;
mod systems;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_startup_system(load_assets)
            .add_system(spawn_splash.in_schedule(OnEnter(AppState::Splash)))
            .add_system(despawn_splash.in_schedule(OnExit(AppState::Splash)))
            .add_system(check_asset_loading.in_set(OnUpdate(AppState::Splash)));
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

pub fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut assets: ResMut<AssetList>,
) {
    let font_regular: Handle<Font> = server.load("fonts/FiraCode-Regular.ttf");
    let font_bold: Handle<Font> = server.load("fonts/FiraCode-Bold.ttf");
    let font_light: Handle<Font> = server.load("fonts/FiraCode-Light.ttf");

    let npc_data: Handle<TomlAsset> = server.load("data/npcs.toml");

    // Push assets to be loaded.
    assets.0.push(font_regular.clone_untyped());
    assets.0.push(font_bold.clone_untyped());
    assets.0.push(font_light.clone_untyped());
    assets.0.push(npc_data.clone_untyped());

    commands.insert_resource(DataAssets { handle: npc_data });
}

pub fn check_asset_loading(
    server: Res<AssetServer>,
    assets: Res<AssetList>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            info!("Loaded {} assets.", assets.0.len());
            next_state.set(AppState::Menu);
        }
        LoadState::Failed => {
            error!("Failed to load assets.");
        }
        _ => {}
    };
}
