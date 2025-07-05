use std::marker::PhantomData;

use bevy::prelude::*;
use edger_bevy_view::prelude::*;

use crate::prelude::*;

#[derive(Event, Debug)]
pub struct DoRootLayoutEvent<TE, T>
where
    TE: LayoutEnv,
    T: View<TE>,
{
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: T,
    pub layout: LayoutData,
}
impl<TE, T> DoRootLayoutEvent<TE, T>
where
    TE: LayoutEnv + Send + Sync + 'static,
    T: View<TE> + Component,
{
    pub fn new(entity: Entity, view: &T, layout: &LayoutData) -> Self {
        Self {
            env: PhantomData,
            entity,
            view: view.clone(),
            layout: layout.clone(),
        }
    }
    pub fn on_root_layout_changed(query: LayoutChangedQuery<T>, mut evts: EventWriter<Self>) {
        for (entity, view, layout) in query.iter() {
            if layout.size.width > 0.0 && layout.size.height > 0.0 {
                if view.log_layout_changed() {
                    println!(
                        "<{}>::on_root_layout_changed({:#?})",
                        std::any::type_name::<T>(),
                        layout
                    );
                }
                evts.write(Self::new(entity, view, layout));
            }
        }
    }
    pub fn on_window_resized(
        mut commands: Commands,
        mut evts: EventReader<WindowResizedEvent>,
        app_state: Res<AppState>,
        view_query: RootViewQuery<T>,
        mut layout_query: LayoutQuery,
    ) {
        let mut resized = false;
        for _evt in evts.read() {
            resized = true;
        }
        if resized {
            let constraint =
                LayoutConstraint::from((app_state.window_width, app_state.window_height));
            for (entity, view) in view_query.iter() {
                let layout_data = view.calc_root_layout(&mut commands, constraint);
                view.set_layout_data(&mut layout_query, entity, layout_data)
            }
        }
    }
    pub fn on_added(
        mut commands: Commands,
        app_state: Res<AppState>,
        view_query: RootViewAddedQuery<T>,
        mut layout_query: LayoutQuery,
    ) {
        let constraint =
            LayoutConstraint::from((app_state.window_width, app_state.window_height));
        for (entity, view) in view_query.iter() {
            let layout_data = view.calc_root_layout(&mut commands, constraint);
            view.set_layout_data(&mut layout_query, entity, layout_data)
        }
    }
    pub fn setup(app: &mut App) {
        app.add_event::<Self>();
        app.add_systems(Update, (
            Self::on_root_layout_changed,
            Self::on_window_resized,
            Self::on_added,
        ));
    }
}

