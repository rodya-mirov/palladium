use super::*;

use specs::RunNow;

pub fn make_quit_panel() -> impl Panel {
    let text = ["Quit game?", "Your progress will not be saved!"]
        .into_iter()
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
    .with_option("[Cancel]".to_string(), |_world: &mut World| vec![PanelAction::CloseCurrentPanel])
    .with_option("[Quit]".to_string(), |world| {
        systems::QuitSystem {}.run_now(&mut world.res);

        vec![PanelAction::CloseCurrentPanel]
    })
    .build(Vector::new(50., 50.))
}
