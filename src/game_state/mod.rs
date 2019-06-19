use quicksilver::{lifecycle::Window, prelude::*};
use specs::World;

use super::*;

use panel::{GamePanel, Panel, PanelAction};

pub struct MainState {
    world: World,
    panels: Vec<Box<dyn Panel>>,
}

pub const FONT_MONONOKI_PATH: &str = "fonts/mononoki/mononoki-Regular.ttf";
pub const FONT_SQUARE_PATH: &str = "fonts/square/square.ttf";

impl State for MainState {
    fn new() -> QsResult<Self> {
        let mut world = World::new();
        let game_panel: GamePanel = GamePanel::new(&mut world);

        Ok(MainState {
            world,
            panels: vec![Box::new(game_panel)],
        })
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        if self.panels.is_empty() {
            window.close();
            return Ok(());
        }

        self.world.add_resource(*window.keyboard());

        let mut actions_by_panel = Vec::with_capacity(self.panels.len());

        let mut actual_panels = std::mem::replace(&mut self.panels, Vec::new());
        let last_ind = actual_panels.len() - 1;
        for (i, panel) in actual_panels.iter_mut().enumerate() {
            let panel_actions = panel
                .update_self(&mut self.world, i == last_ind)
                .expect("Update needs to not error");
            actions_by_panel.push(panel_actions);
        }

        let mut kb_update_actions = actual_panels
            .get_mut(last_ind)
            .expect("This should always exist because the vec is nonempty")
            .do_key_input(&mut self.world, window.keyboard())
            .expect("KB input needs to not error");

        actions_by_panel
            .get_mut(last_ind)
            .expect("This should always exist")
            .append(&mut kb_update_actions);

        std::mem::replace(&mut self.panels, actual_panels);

        self.process_panel_actions(actions_by_panel);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        if self.panels.is_empty() {
            window.close();
            return Ok(());
        }

        let mut actions_by_panel = Vec::with_capacity(self.panels.len());
        let mut actual_panels = std::mem::replace(&mut self.panels, Vec::new());
        for panel in actual_panels.iter_mut() {
            let panel_render_actions = panel.render_self(&mut self.world, window).expect("Rendering needs to not error");
            window.flush()?;
            actions_by_panel.push(panel_render_actions);
        }
        std::mem::replace(&mut self.panels, actual_panels);

        self.process_panel_actions(actions_by_panel);

        Ok(())
    }
}

impl MainState {
    fn process_panel_actions(&mut self, actions_by_panel: Vec<Vec<PanelAction>>) {
        let mut new_panels = Vec::new();

        for panel_actions in actions_by_panel {
            let old_panel = self.panels.remove(0);

            let mut keep_old = true;
            let mut next_panel_index = new_panels.len();
            for panel_action in panel_actions {
                match panel_action {
                    PanelAction::CloseCurrentPanel => {
                        keep_old = false;
                    }
                    PanelAction::AddPanelAbove(new_panel) => {
                        new_panels.push(new_panel);
                    }
                    PanelAction::AddPanelBehind(new_panel) => {
                        new_panels.insert(next_panel_index, new_panel);
                        next_panel_index += 1;
                    }
                }
            }
            if keep_old {
                new_panels.insert(next_panel_index, old_panel);
            }
        }

        self.panels = new_panels;
    }
}

pub fn is_loaded<T>(asset: &mut Asset<T>) -> QsResult<bool> {
    let mut is_loaded = false;
    asset.execute(|_t| {
        is_loaded = true;
        Ok(())
    })?;
    Ok(is_loaded)
}

pub fn all_loaded<'a, T>(assets: &mut [&'a mut Asset<T>]) -> QsResult<bool> {
    for asset in assets.iter_mut() {
        let good = is_loaded(asset)?;
        if !good {
            return Ok(false);
        }
    }

    Ok(true)
}
