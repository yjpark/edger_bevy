use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext, io::Reader};
use bevy::reflect::TypePath;
use bevy::utils::BoxedFuture;

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

    fn load<'a>(
        &'a self,
        reader: &'a mut dyn Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, LoadResult> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let text = String::from_utf8(bytes.to_vec())?;
            let asset = MarkDownAsset::from(text);
            Ok(asset)
        })
    }
    fn extensions(&self) -> &[&str] {
        &["md"]
    }
}
