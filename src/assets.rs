use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AssetsStates {
    #[default]
    Init,
    Loading,
    Loaded,
}

pub trait PreloadAssets : AssetCollection {
    fn app_assets(&self) -> Vec<UntypedHandle> {
        vec![]
    }
    fn setup_keys(&self, assets_keys: &mut DynamicAssets);
}

fn setup_asset_keys<A: PreloadAssets>(
    assets: ResMut<A>,
    mut asset_keys: ResMut<DynamicAssets>,
    mut state: ResMut<NextState<AssetsStates>>,
) {
    assets.setup_keys(&mut asset_keys);
    state.set(AssetsStates::Loading);
}

fn add_assets_to_app_state<A: PreloadAssets>(
    assets: Res<A>,
    mut app_state: ResMut<crate::app::state::AppState>,
) {
    for asset in assets.app_assets().iter() {
        app_state.assets.push(asset.clone());
    }
}

fn preload_assets<A: PreloadAssets>(app: &mut App) {
    app.add_systems(OnEnter(AssetsStates::Init), (
        setup_asset_keys::<A>,
    ));
    app.add_collection_to_loading_state::<_, A>(AssetsStates::Loading);
    app.add_systems(OnEnter(AssetsStates::Loaded), (
        add_assets_to_app_state::<A>,
    ));
}

pub fn init_preload_assets<A: PreloadAssets + FromWorld>(app: &mut App) {
    app.init_resource::<A>();
    preload_assets::<A>(app);
}

pub fn insert_preload_assets<A: PreloadAssets>(app: &mut App, assets: A) {
    app.insert_resource(assets);
    preload_assets::<A>(app);
}

pub fn add_assets_loaded_systems<M>(
    app: &mut App,
    systems: impl IntoScheduleConfigs<bevy::ecs::system::ScheduleSystem, M>,
) {
    app.add_systems(OnEnter(AssetsStates::Loaded), systems);
}

pub fn register_file_asset<K: Into<String>>(asset_keys: &mut DynamicAssets, key: K, path: String) {
    asset_keys.register_asset(key, Box::new(StandardDynamicAsset::File {
        path
    }));
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetsStates>();

        app.add_loading_state(
            LoadingState::new(AssetsStates::Loading)
                .continue_to_state(AssetsStates::Loaded)
        );
    }
}