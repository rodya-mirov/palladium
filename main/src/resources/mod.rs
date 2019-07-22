use super::*;

use std::collections::VecDeque;

#[derive(Clone)]
pub struct RenderStale(pub bool);

impl Default for RenderStale {
    fn default() -> Self {
        RenderStale(true)
    }
}

#[derive(Clone, Default)]
pub struct SavedStates {
    pub saves: Vec<SaveGameData>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone)]
pub struct SaveGameData {
    // components, saved
    pub world_state: Vec<u8>,
    // resources, saved
    pub resources: Vec<u8>,
}

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
    pub callbacks: Vec<Callback>,
}

#[derive(Clone, Debug, Default)]
pub struct Callbacks(Vec<Callback>);

impl std::ops::Deref for Callbacks {
    type Target = Vec<Callback>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Callbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum TakeDecision<A, B> {
    Take(A),
    Leave(B),
}

impl Callbacks {
    pub fn take_some<F, T>(&mut self, mut take_mapper: F) -> Vec<T>
    where
        F: FnMut(Callback) -> TakeDecision<T, Callback>,
    {
        let callbacks = std::mem::replace(&mut self.0, Vec::new());

        let mut keep = Vec::new();
        let mut out = Vec::new();

        for cb in callbacks {
            match take_mapper(cb) {
                TakeDecision::Leave(b) => keep.push(b),
                TakeDecision::Take(a) => out.push(a),
            }
        }

        std::mem::replace(&mut self.0, keep);
        out
    }
}

#[derive(Clone, Debug)]
pub enum Callback {
    // start a dialogue
    StartDialogue(DialogueState),
    // end current dialogue, return to normal gameplay
    EndDialogue,
    // rollup for hack callbacks
    Hack(HackCallback),
    // rollup for talk callbacks
    Talk(TalkCallback),
    // request to save the game; handled by SaveSystem
    SaveGame,
    // request to load the game; handled by LoadSystem
    LoadGame,
    // request to quit the game
    QuitGame,
}

#[derive(Clone, Debug)]
pub enum HackCallback {
    // when you select a hack target and you need to see a more specific menu about
    // how you want to hack it
    ChooseHackTarget { entity: Entity },
    // when you actually select the hack type and start hacking
    InitiateHack { target: HackTarget, turn_duration: usize },
}

#[derive(Clone, Debug)]
pub enum TalkCallback {
    ChooseTalkTarget { entity: Entity },
}

#[derive(Clone, Debug)]
pub struct HackTarget {
    pub entity: Entity,
    pub hack_type: HackType,
}

#[derive(Clone, Debug)]
pub enum HackType {
    // things must be compromised before then can be messed with
    Compromise,
    // set the behavior of a door
    Door { new_door_behavior: components::DoorBehavior },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
    pub oxygen_meter_offset_px: Vector,
}

impl Default for GameMapRenderParams {
    fn default() -> Self {
        GameMapRenderParams {
            font_width: 20.0,
            map_offset: Vector::new(50.0, 50.0),
            tile_size_px: Vector::new(20.0, 20.0),
            controls_image_offset_px: Vector::new(30.0, 30.0),
            oxygen_meter_offset_px: Vector::new(30.0, 60.0),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueuedPlayerActions {
    pub action_queue: VecDeque<QueuedPlayerAction>,
}

#[derive(Clone, Debug)]
pub enum QueuedPlayerAction {
    Wait,
    Hack { target: HackTarget },
}
