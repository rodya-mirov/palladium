use quicksilver::prelude::*;

use super::*;

pub fn make_license_panel() -> impl Panel {
    let text = [
        "Mononoki font by Matthias Tellen, terms: Open Font License 1.1",
        "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join("\n");

    MenuPanelBuilder::new(
        text,
        crate::game_state::FONT_MONONOKI_PATH,
        20.,
        Color::WHITE,
        Color {
            r: 0.7,
            g: 0.7,
            b: 0.7,
            a: 1.,
        },
    )
    .build(Vector::new(50., 50.))
}
