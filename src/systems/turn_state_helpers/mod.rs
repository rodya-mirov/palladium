use super::*;

use resources::NpcMoves;

// how many update frames after a manual move until NPC can move
// essentially player can do 30 / (this many) taps per second (assuming we're running at 60fps)
// so keep that in mind
const TICKS_PER_MANUAL_TURN: usize = 6;

pub fn yield_to_npc(npc_moves: &mut NpcMoves) {
    yield_moves_to_npc(npc_moves, 1);
}

pub fn yield_moves_to_npc(npc_moves: &mut NpcMoves, num_moves: usize) {
    npc_moves.npc_moves_remaining += num_moves;
    npc_moves.ticks_till_next_npc_move = TICKS_PER_MANUAL_TURN;
    npc_moves.move_was_made = true;
}

pub fn timestep(npc_moves: &mut NpcMoves) {
    if npc_moves.ticks_till_next_npc_move > 0 {
        npc_moves.ticks_till_next_npc_move -= 1;
    } else if npc_moves.npc_moves_remaining > 0 {
        // Note: we don't check if an NPC actually DID move;
        // all systems which move NPCs should not waste ticks to do so,
        // so it's fine to take their turn away.
        // This may change in the future, if we want to allow for
        // some processing time in the background or something.
        npc_moves.npc_moves_remaining -= 1;
        npc_moves.ticks_till_next_npc_move = TICKS_PER_MANUAL_TURN;
    }
    npc_moves.move_was_made = false;
}
