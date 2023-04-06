use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{Handle, Resource},
    reflect::TypeUuid,
    utils::BoxedFuture,
};

#[derive(Debug, Resource)]
pub struct DataAssets {
    pub handle: Handle<TomlAsset>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
pub struct TomlAsset(pub String);

#[derive(Default)]
pub struct TomlLoader;

impl AssetLoader for TomlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let asset = TomlAsset(data_str.into());
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
