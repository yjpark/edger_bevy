use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader};
use bevy::reflect::TypePath;

#[derive(Debug, Asset, TypePath)]
pub struct MarkDownAsset {
    pub text: String,
}

impl From<String> for MarkDownAsset {
    fn from(v: String) -> Self {
        Self { text: v }
    }
}

impl From<&str> for MarkDownAsset {
    fn from(v: &str) -> Self {
        Self {
            text: String::from(v),
        }
    }
}

#[derive(Default)]
pub struct MarkDownAssetLoader;

pub type LoadError = anyhow::Error;
pub type LoadResult = anyhow::Result<MarkDownAsset, LoadError>;

impl AssetLoader for MarkDownAssetLoader {
    type Asset = MarkDownAsset;
    type Settings = ();
    type Error = LoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes.to_vec())?;
        let asset = MarkDownAsset::from(text);
        Ok(asset)
    }
    fn extensions(&self) -> &[&str] {
        &["md"]
    }
}
