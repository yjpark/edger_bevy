use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;

use bevy_egui::{egui::{FontDefinitions, FontData, FontFamily, FontId, Style, TextStyle}, EguiContexts};

lazy_static! {
    static ref EGUI_DEFAULT_FONT: RwLock<EguiFont> = RwLock::new(EguiFont::default());
}

#[derive(Clone, Debug, Default)]
pub struct EguiFont {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl EguiFont {
    pub fn has_default() -> bool {
        EGUI_DEFAULT_FONT.read().unwrap().data.is_some()
    }

    pub fn set_default(name: String, data: Vec<u8>) {
        let mut font = EGUI_DEFAULT_FONT.write().unwrap();
        font.name = name;
        font.data = Some(data);
    }

    pub fn get_defalut() -> Self {
        EGUI_DEFAULT_FONT.read().unwrap().clone()
    }

    pub fn get_default_definitions() -> FontDefinitions {
        let font = Self::get_defalut();
        let mut fonts = FontDefinitions::default();
        if let Some(data) = font.data {
            let name = font.name;
            let font_data = FontData::from_owned(data);
            fonts.font_data.insert(name.clone(), Arc::new(font_data));
            fonts.families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, name.clone());
            fonts.families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, name.clone());
        }
        fonts
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EguiFontSizes {
    pub small: f32,
    pub body: f32,
    pub button: f32,
    pub heading: f32,
    pub mono: f32,
}

impl Default for EguiFontSizes {
    fn default() -> Self {
        Self {
            small: 14.0,
            body: 16.0,
            button: 18.0,
            heading: 24.0,
            mono: 16.0,
        }
    }
}

impl EguiFontSizes {
    pub const BIGGER: Self = Self {
        small: 16.0,
        body: 18.0,
        button: 20.0,
        heading: 26.0,
        mono: 18.0,
    };
}

impl EguiFontSizes {
    pub fn apply_style(&self, style: &mut Style) {
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(self.small, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(self.body, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(self.button, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(self.heading, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(self.mono, FontFamily::Monospace),
        );
    }

    pub fn apply_context(&self, egui_ctx: &mut EguiContexts) {
        let fonts = EguiFont::get_default_definitions();
        let ctx = egui_ctx.ctx_mut();
        ctx.set_fonts(fonts);
        let mut style: Style = (*ctx.style()).clone();
        self.apply_style(&mut style);
        ctx.set_style(style);
    }
}

