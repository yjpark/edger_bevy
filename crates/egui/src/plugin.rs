use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContextSettings;

pub struct EguiPlugin;

impl Plugin for EguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin::default());
        crate::prelude::EasyLinkEvent::setup(app);
        app.init_asset_loader::<crate::prelude::EguiFontAssetLoader>();

        app.add_systems(Update, update_scale_factor);
    }
}

fn update_scale_factor(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut egui_settings: Query<&mut EguiContextSettings>,
) {
    if let Ok(window) = window_query.single() {
        let scale_factor = 1.0 / window.scale_factor();
        let mut settings = egui_settings.single_mut().unwrap();
        if scale_factor != settings.scale_factor {
            println!(
                "egui scale_factor changed:() {} -> {}",
                settings.scale_factor, scale_factor
            );
            settings.scale_factor = scale_factor;
        }
    }
}

