use bevy::{prelude::*, utils::HashMap};

use crate::utils::text_asset_loader::{DataAssets, TomlAsset};

use super::components::Npc;

pub fn get_npcs(toml_assets: Res<Assets<TomlAsset>>, data_assets: Res<DataAssets>) {
    // for npc in query.iter() {
    //     println!("NPC: {:?}", npc);
    // }

    let data = toml_assets
        .get(&data_assets.handle)
        .expect("Not a valid asset!");
    info!("{:?}", data);

    let npcs: HashMap<&str, Npc> = toml::from_str(&data.0).expect("Unable to parse data!");
    info!("{:?}", npcs);
}
