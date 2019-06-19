#![allow(dead_code)] // TODO: remove this

use super::*;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, FontStyle, Image},
    input::{ButtonState, Key, Keyboard},
    Future,
};
use specs::RunNow;

mod license_panel;
mod quit_panel;

pub use license_panel::make_license_panel;
pub use quit_panel::make_quit_panel;

fn force_max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else if a > b {
        a
    } else {
        b
    }
}

pub type MenuCallback = FnMut(&mut World) -> Vec<PanelAction>;

pub struct MenuPanel {
    main_text: Asset<Image>,
    options: Vec<MenuOption>,
    padding: Vector,
    selection_index: usize,
}

pub struct MenuPanelBuilder {
    main_text: String,
    font: &'static str,
    font_size: f32,
    selected_color: Color,
    unselected_color: Color,
    options: Vec<MenuOptionBuilder>,
}

struct MenuOptionBuilder {
    text: String,
    callback: Box<dyn FnMut(&mut World) -> Vec<PanelAction>>,
}

impl MenuPanelBuilder {
    pub fn new(main_text: String, font: &'static str, font_size: f32, selected_color: Color, unselected_color: Color) -> MenuPanelBuilder {
        MenuPanelBuilder {
            main_text,
            font,
            font_size,
            selected_color,
            unselected_color,
            options: Vec::new(),
        }
    }

    fn add_option<F: 'static>(&mut self, text: String, callback: F)
    where
        for<'r> F: FnMut(&'r mut World) -> Vec<PanelAction>,
    {
        let callback: Box<dyn FnMut(&mut World) -> Vec<PanelAction>> = Box::new(callback);
        self.options.push(MenuOptionBuilder { text, callback });
    }

    pub fn with_option<F: 'static>(mut self, text: String, callback: F) -> Self
    where
        for<'r> F: FnMut(&'r mut World) -> Vec<PanelAction>,
    {
        self.add_option(text, callback);
        self
    }

    pub fn build(mut self, padding: Vector) -> MenuPanel {
        if self.options.is_empty() {
            let callback = |_world: &mut World| vec![PanelAction::CloseCurrentPanel];
            self.add_option("[Continue]".to_owned(), callback);
        }
        let main_text = load_image(self.main_text, self.font, self.font_size, self.selected_color);

        let mut options = Vec::new();

        for option in self.options {
            options.push(MenuOption {
                selected_text: load_image(option.text.clone(), self.font, self.font_size, self.selected_color),
                unselected_text: load_image(option.text, self.font, self.font_size, self.unselected_color),
                on_choose: option.callback,
            })
        }

        MenuPanel {
            main_text,
            options,
            padding,
            selection_index: 0,
        }
    }
}

fn load_image(text: String, font: &'static str, font_size: f32, color: Color) -> Asset<Image> {
    Asset::new(Font::load(font).and_then(move |font| font.render(&text, &FontStyle::new(font_size, color))))
}

pub struct MenuOption {
    selected_text: Asset<Image>,
    unselected_text: Asset<Image>,
    on_choose: Box<dyn FnMut(&mut World) -> Vec<PanelAction>>,
}

impl MenuOption {
    fn get_text_mut(&mut self, is_selected: bool) -> &mut Asset<Image> {
        if is_selected {
            &mut self.selected_text
        } else {
            &mut self.unselected_text
        }
    }
}

const SELECTION_KEYS: [Key; 3] = [Key::Space, Key::NumpadEnter, Key::Return];
const CLEAR_COLOR: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 0.7,
};
const BG_COLOR: Color = Color {
    r: 0.3,
    g: 0.6,
    b: 0.5,
    a: 1.,
};

impl Panel for MenuPanel {
    fn update_self(&mut self, _world: &mut World, _is_active: bool) -> PanelResult {
        Ok(Vec::new())
    }

    fn render_self(&mut self, world: &mut World, window: &mut Window) -> PanelResult {
        let mut images: Vec<&mut Asset<Image>> = Vec::with_capacity(self.options.len() + 1);
        images.push(&mut self.main_text);
        for (i, option) in self.options.iter_mut().enumerate() {
            if i == self.selection_index {
                images.push(&mut option.selected_text);
            } else {
                images.push(&mut option.unselected_text);
            }
        }

        systems::CenteredVerticalImagesRenderer {
            window,
            images: &mut images,
            bg_color: BG_COLOR,
            outside_padding: Vector::new(50, 50),
            internal_padding: Vector::new(20, 20),
        }
        .run_now(&mut world.res);

        Ok(Vec::new())
    }

    fn do_key_input(&mut self, world: &mut World, keyboard: &Keyboard) -> PanelResult {
        let num_options = self.options.len(); // should never be 0, and should be smallish

        if keyboard[Key::Up] == ButtonState::Pressed {
            self.selection_index = (self.selection_index + num_options - 1) % num_options;
        }
        if keyboard[Key::Down] == ButtonState::Pressed {
            self.selection_index = (self.selection_index + 1) % num_options;
        }

        let mut actions = Vec::new();

        if SELECTION_KEYS.iter().any(|&key| keyboard[key] == ButtonState::Pressed) {
            let mut actual_options = std::mem::replace(&mut self.options, Vec::new());
            let on_choose = &mut actual_options
                .get_mut(self.selection_index)
                .expect("Selection index should always be valid")
                .on_choose;
            let mut new_actions = on_choose(world);
            actions.append(&mut new_actions);
            std::mem::replace(&mut self.options, actual_options);
        }

        Ok(actions)
    }
}
