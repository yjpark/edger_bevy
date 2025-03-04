use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;

use super::font::EguiFont;

#[derive(Default)]
pub struct EguiFontAssetLoader;

pub type LoadError = anyhow::Error;
pub type LoadResult = anyhow::Result<Font, LoadError>;

impl AssetLoader for EguiFontAssetLoader {
    type Asset = Font;
    type Settings = ();
    type Error = LoadError;


    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        if !EguiFont::has_default() {
            let filename = load_context.path().file_name()
            .and_then(|x| x.to_str())
            .map(|x| x.replace(".font", ""))
            .unwrap_or("bevy_egui_font".to_owned());
            let data = bytes.to_vec();
            EguiFont::set_default(filename.to_owned(), data);
        }
        let font = Font::try_from_bytes(bytes.to_vec())?;
        Ok(font)
    }

    fn extensions(&self) -> &[&str] {
        &["egui"]
    }
}
