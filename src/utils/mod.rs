use bevy::prelude::*;

use self::text_asset_loader::{TomlAsset, TomlLoader};

pub mod text_asset_loader;
pub mod vectors;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        // Load custom asset loader and asset type.
        app.init_asset_loader::<TomlLoader>()
            .add_asset::<TomlAsset>();
    }
}
