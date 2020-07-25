use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{BaseColor, BorderStyle, Palette, Theme};

fn get_palette() -> Palette {
    let mut palette = Palette::default();
    let colors = vec![
        (Background, Light(BaseColor::Black)),
        (View, Dark(BaseColor::Black)),
        (Primary, Dark(BaseColor::White)),
        (Secondary, Dark(BaseColor::Black)),
        (Tertiary, Dark(BaseColor::Green)),
        (Highlight, Dark(BaseColor::Red)),
    ];
    palette.extend(colors);
    palette
}

pub fn get() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::None;
    theme.palette = get_palette();
    theme
}
