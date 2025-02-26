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
        ..Default::default()
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

pub fn set_size(text_font: &mut TextFont, font_size: f32) {
    text_font.font_size = font_size;
}

pub fn set_color(text_color: &mut TextColor, color: Color) {
    text_color.0 = color;
}

pub fn set_size_color(text_font: &mut TextFont, text_color: &mut TextColor, font_size: f32, color: Color) {
    set_size(text_font, font_size);
    set_color(text_color, color);
}

pub fn set_value(text: &mut Text2d, v: String) {
    text.0 = v;
}
