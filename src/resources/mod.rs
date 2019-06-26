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
    ChooseHackTarget {
        entity: Entity,
    },
    InitiateHack {
        entity: Entity,
        target: HackTarget,
        turn_duration: usize,
    },
}

#[derive(Clone, Debug)]
pub enum HackTarget {
    Door { new_hack_state: components::DoorHackState },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameClock {
    pub hours: usize,
    pub minutes: usize,
    pub seconds: usize,
}

impl Default for GameClock {
    fn default() -> Self {
        GameClock {
            hours: 15,
            minutes: 12,
            seconds: 40,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NpcMoves {
    pub npc_moves_remaining: usize,
    pub ticks_till_next_npc_move: usize,
    // Essentially, whether "response" systems should run
    pub move_was_made: bool,
}

impl NpcMoves {
    pub fn player_can_move(&self) -> bool {
        self.npc_moves_remaining == 0 && self.ticks_till_next_npc_move == 0
    }

    #[allow(dead_code)] // TODO: left in for doc reasons but remove this when we have NPCs
    pub fn npc_can_move(&self) -> bool {
        self.npc_moves_remaining > 0 && self.ticks_till_next_npc_move == 0
    }
}

impl Default for NpcMoves {
    fn default() -> Self {
        NpcMoves {
            npc_moves_remaining: 0,
            ticks_till_next_npc_move: 0,
            // start with true so various update steps will run
            // in the first timestep, as part of initialization
            move_was_made: true,
        }
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
