use bevy::prelude::*;

pub fn calc_name(name: String) -> Name {
    if name.len() <= 32 {
        name.as_str().into()
    } else {
        name.as_str()[..32].into()
    }
}

pub fn spawn_child_bundle<T: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    bundle: T,
) -> Entity {
    let child_entity = commands.spawn(bundle).id();
    commands.entity(entity).add_children(&[child_entity]);
    child_entity
}
