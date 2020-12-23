use winrt::*;
include_bindings!();

use windows::ui::{self, view_management};

fn get_color_value(color_type: view_management::UIColorType) -> ui::Color {
    if let Ok(settings) = view_management::UISettings::new() {
        if let Ok(color) = settings.get_color_value(color_type) {
            return color;
        }
    }
    ui::Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

pub fn accent_rgba8() -> ui::Color {
    get_color_value(view_management::UIColorType::Accent)
}

pub fn background_rgba8() -> ui::Color {
    get_color_value(view_management::UIColorType::Background)
}

pub fn foreground_rgba8() -> ui::Color {
    get_color_value(view_management::UIColorType::Foreground)
}
