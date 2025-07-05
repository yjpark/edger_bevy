use bevy::prelude::*;

pub struct EguiPlugin;

impl Plugin for EguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin::default());
        crate::prelude::EasyLinkEvent::setup(app);
        app.init_asset_loader::<crate::prelude::EguiFontAssetLoader>();
    }
}