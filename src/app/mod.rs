use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::prelude::*;

pub mod state;
pub mod events;

pub fn run_2d_app<F>(extra: F)
where
    F: Fn(&mut App)
{
    let mut app = App::new();

    //app.insert_resource(Msaa::Sample4);

    //https://github.com/bevyengine/bevy/issues/10157
    #[cfg(target_arch = "wasm32")]
    app.insert_resource(AssetMetaCheck::Never);

    // https://github.com/ostwilkens/bevy_web_fullscreen/pull/9
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            // https://github.com/bevyengine/bevy/pull/11057
            // fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(UtilsPlugin);

    #[cfg(feature = "assets")]
    app.add_plugins(AssetsPlugin);

    events::add_app_events(&mut app);

    #[cfg(feature = "shape")]
    app.add_plugins(ShapePlugin);

    #[cfg(feature = "view")]
    app.add_plugins(ViewPlugin);

    #[cfg(feature = "egui")]
    app.add_plugins(EguiPlugin);

    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    app.init_resource::<AppState>();

    app.add_systems(Startup, (
        setup_window_size,
        setup_2d_camera,
    ));

    app.add_systems(Update, (
        on_window_resized,
        update_window_scale_factor,
    ));

    extra(&mut app);
    app.run();
}

fn setup_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_window_size(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state: ResMut<AppState>
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let (width, height) = (window.width(), window.height());

    println!("setup_window_size(): {} {} ", width, height);
    app_state.window_width = width;
    app_state.window_height = height;
}

fn on_window_resized(
    window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut evts: EventReader<WindowResized>,
    mut app_state: ResMut<AppState>,
    mut window_resized_evts: EventWriter<WindowResizedEvent>,
) {
    let Ok((window_entity, window)) = window_query.get_single() else {
        return;
    };
    for evt in evts.read() {
        if window_entity != evt.window { continue }
        if evt.width as usize != app_state.window_width as usize
            || evt.height as usize != app_state.window_height as usize
        {
            println!(
                "on_window_resized(): {} {} -> {} {} ",
                app_state.window_width, app_state.window_height, evt.width, evt.height
            );
            let resized_evt = WindowResizedEvent::new(&app_state);
            app_state.window_width = evt.width;
            app_state.window_height = evt.height;
            app_state.scale_factor_override = window.resolution.scale_factor_override();
            window_resized_evts.send(resized_evt);
        }
    }
}

fn update_window_scale_factor(
    //mut _egui_settings: ResMut<EguiSettings>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state: ResMut<AppState>,
) {
    if let Ok(window) = window_query.get_single() {
        let scale_factor = window.scale_factor();
        if scale_factor != app_state.window_scale_factor {
            println!(
                "scale_factor changed:() {} -> {}",
                app_state.window_scale_factor, scale_factor
            );
            app_state.window_scale_factor = scale_factor;
            /*
             * egui_settings.scale_factor = 1.0 / scale_factor;
             */
        }
    }
}