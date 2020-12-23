use crate::base;

macro_rules! convert_to_piet {
    ( $old_name:ident, $new_name:ident ) => {
        pub fn $new_name() -> piet_rs::Color {
            let color = base::$old_name();
            piet_rs::Color::rgba8(color.r, color.g, color.b, color.a)
        }
    };
}

convert_to_piet!(accent_rgba8, accent_color);
convert_to_piet!(background_rgba8, background_color);
convert_to_piet!(foreground_rgba8, foreground_color);
