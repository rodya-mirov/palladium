use super::*;

use std::collections::HashMap;

use rand::{Rng, SeedableRng};

use components::{BlocksMovement, HasPosition, NPC};
use resources::NpcMoves;

pub struct NpcMoveSystem;

#[derive(SystemData)]
pub struct NpcMoveSystemData<'a> {
    npc: ReadStorage<'a, NPC>,
    has_position: WriteStorage<'a, HasPosition>,
    blocks_moves: ReadStorage<'a, BlocksMovement>,
    entities: Entities<'a>,
    npc_moves: Read<'a, NpcMoves>,
}

impl<'a> System<'a> for NpcMoveSystem {
    type SystemData = NpcMoveSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.npc_moves.npc_can_move() {
            return;
        }

        // TODO: make this a resource
        let mut rng = crate::rng::PalladRng::seed_from_u64(34431);

        let mut blocks: HashMap<TilePos, u32> = get_all_blocks(&data.has_position, &data.blocks_moves);
        let mut choices = Vec::with_capacity(5);

        for (_, has_position, entity) in (&data.npc, &mut data.has_position, &data.entities).join() {
            choices.clear();

            let pos = has_position.position;

            for _ in 0..3 {
                choices.push(TilePos { x: 0, y: 0 });
            }

            maybe_note_move_choice(pos, TilePos { x: -1, y: 0 }, &mut choices, &blocks);
            maybe_note_move_choice(pos, TilePos { x: 1, y: 0 }, &mut choices, &blocks);
            maybe_note_move_choice(pos, TilePos { x: 0, y: -1 }, &mut choices, &blocks);
            maybe_note_move_choice(pos, TilePos { x: 0, y: 1 }, &mut choices, &blocks);

            let choice_ind = rng.gen_range(0, choices.len());
            let choice = *choices.get(choice_ind).expect("Choice index should be guaranteed valid");

            if data.blocks_moves.get(entity).is_some() {
                *blocks.get_mut(&pos).expect("Should be an entry at position since this blocks") -= 1;
                *blocks.entry(pos + choice).or_insert(0) += 1;
            }

            has_position.position += choice;
        }
    }
}

fn maybe_note_move_choice(position: TilePos, change: TilePos, choices_vec: &mut Vec<TilePos>, blocks: &HashMap<TilePos, u32>) {
    let new_pos = position + change;
    if *blocks.get(&new_pos).unwrap_or(&0) == 0 {
        choices_vec.push(change);
    }
}

fn get_all_blocks<'a>(
    has_position: &WriteStorage<'a, HasPosition>,
    blocks_moves: &ReadStorage<'a, BlocksMovement>,
) -> HashMap<TilePos, u32> {
    let mut block_counts = HashMap::new();

    for (hp, _) in (has_position, blocks_moves).join() {
        *block_counts.entry(hp.position).or_insert(0) += 1;
    }

    block_counts
}
