use bevy::{prelude::*, sprite::Anchor};

pub fn spawn(
    commands: &mut Commands,
    entity: Entity,
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    justify: JustifyText,
    anchor: Anchor,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {
    let text_font = TextFont {
        font,
        font_size,
    };
    let text_entity = commands
        .spawn((
            Text2d::new(text),
            TextLayout::new_with_justify(justify),
            text_font,
            TextColor::from(color),
            Transform::from_xyz(x, y, z),
            anchor,
        ))
        .id();
    commands.entity(entity).add_children(&[text_entity]);
    text_entity
}
pub fn set_size(text: &mut Text, font_size: f32) {
    for section in text.sections.iter_mut() {
        section.style.font_size = font_size;
    }
}
pub fn set_color(text: &mut Text, color: Color) {
    for section in text.sections.iter_mut() {
        section.style.color = color;
    }
}
pub fn set_size_color(text: &mut Text, font_size: f32, color: Color) {
    for section in text.sections.iter_mut() {
        section.style.font_size = font_size;
        section.style.color = color;
    }
}
pub fn set_value(text: &mut Text, v: String) {
    for section in text.sections.iter_mut() {
        section.value = v;
        return;
    }
}
