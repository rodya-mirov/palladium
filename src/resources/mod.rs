use super::*;

#[derive(Clone, Debug)]
pub struct DialogueStateResource {
    pub is_initialized: InitializationState,
    pub state: Option<DialogueState>,
}

impl Default for DialogueStateResource {
    fn default() -> Self {
        DialogueStateResource {
            // if dialogue is off, state is nothing, it's fine
            is_initialized: InitializationState::Finished,
            state: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InitializationState {
    NotStarted,
    Started,
    Finished,
}

#[derive(Clone, Debug)]
pub struct DialogueState {
    pub main_text: String,
    pub selected_index: usize,
    pub options: Vec<DialogueOptionState>,
}

#[derive(Clone, Debug)]
pub struct DialogueOptionState {
    pub selected_text: String,
    pub unselected_text: String,
    pub callbacks: Vec<DialogueCallback>,
}

#[derive(Clone, Debug)]
pub enum DialogueCallback {
    // end current dialogue, return to normal gameplay
    EndDialogue,
    Hack(HackDialogueCallback),
    QuitGame,
}

#[derive(Clone, Debug)]
pub enum HackDialogueCallback {
    ChooseHackTarget(Entity),
    InitiateHack(Entity, HackTarget),
}

#[derive(Clone, Debug)]
pub enum HackTarget {
    Door(components::DoorHackState), // door(new_hack_state)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PlayerHasMoved {
    pub player_has_moved: bool,
}

impl Default for PlayerHasMoved {
    fn default() -> Self {
        // starts with "true" so various update steps will run
        // in the first timestep, to help initialize the world
        PlayerHasMoved { player_has_moved: true }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyboardFocus {
    GameMap,
    Dialogue,
}

impl Default for KeyboardFocus {
    fn default() -> Self {
        KeyboardFocus::GameMap
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GameIsQuit(pub bool);

impl Default for GameIsQuit {
    fn default() -> Self {
        GameIsQuit(false)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameMapDisplayOptions {
    pub display_controls_pane: bool,
    pub show_oxygen_overlay: bool,
}

impl Default for GameMapDisplayOptions {
    fn default() -> Self {
        GameMapDisplayOptions {
            display_controls_pane: true,
            show_oxygen_overlay: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GameMapRenderParams {
    pub font_width: f32,
    pub map_offset: Vector,
    pub tile_size_px: Vector,
    pub controls_image_offset_px: Vector,
}

impl Default for GameMapRenderParams {
    fn default() -> Self {
        GameMapRenderParams {
            font_width: 20.0,
            map_offset: Vector::new(50.0, 50.0),
            tile_size_px: Vector::new(20.0, 20.0),
            controls_image_offset_px: Vector::new(30.0, 30.0),
        }
    }
}
