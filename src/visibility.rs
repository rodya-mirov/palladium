//! Module for handling visibility
//! shamelessly taken from http://journal.stuffwithstuff.com/2015/09/07/what-the-hero-sees/
//! which is a completely wonderful article

use crate::maps::{Map, Square, SquareType, VisibilityType};
use crate::numerics::Float;
use crate::state::TilePos;

fn can_see_through(square: Square) -> bool {
    match square.square_type {
        SquareType::Floor => true,

        SquareType::Door => false,
        SquareType::Wall => false,

        // These are perhaps debatable, but they should never be visible anyway
        // so shouldn't matter
        SquareType::Open => false,
        SquareType::Void => false,
    }
}

#[derive(Copy, Clone, Debug)]
struct Shadow {
    start: Float,
    end: Float,
}

impl Shadow {
    fn contains(self, other: Shadow) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

// only refers to a single octant (!)
#[derive(Default, Debug)]
struct ShadowLine {
    shadows: Vec<Shadow>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Visible {
    IsVisible,
    IsNotVisible,
}

impl ShadowLine {
    fn is_in_shadow(&self, projection: Shadow) -> bool {
        self.shadows.iter().any(|shadow| shadow.contains(projection))
    }

    // precondition: shadow is a nontrivial addition
    fn add_shadow(&mut self, to_insert: Shadow) {
        let shadow_len = self.shadows.len();

        let mut index = 0;

        // linear search for the insertion point; list tends to be very short
        // so that index is the first where to_insert.start <= shadows[index].start
        while index < shadow_len && self.shadows[index].start < to_insert.start {
            index += 1;
        }

        // NB: because of the iteration order (doing individual squares, and doing closer
        // rows before later ones), we actually never need to unify more than two shadows
        // because this square will always fit in the union of two adjacent shadows
        let overlaps_previous = index > 0 && self.shadows[index - 1].end >= to_insert.start;
        let overlaps_next = index < shadow_len && self.shadows[index].start <= to_insert.end;

        if overlaps_next {
            if overlaps_previous {
                // Then we need to unify prev and next into one shadow
                self.shadows[index - 1].end = self.shadows[index].end;
                self.shadows.remove(index);
            } else {
                self.shadows[index].start = to_insert.start;
            }
        } else {
            if overlaps_previous {
                self.shadows[index - 1].end = to_insert.end;
            } else {
                self.shadows.insert(index, to_insert);
            }
        }
    }
}

/// Computes a shadow of a given occlusive block at (row, col)
/// (which is a relative position to the observer). Assumes the
/// thing has already been transformed by octant, so is "in the
/// zeroth octant"
///
/// Params:
/// * rel_pos: position relative to the observer's position
///
/// Precondition: row >= col (N/NE octant), row >= 1
fn project_tile(row: i32, col: i32) -> Shadow {
    // NB: here, row means "up" (so 3 means 3 tiles north)
    // whereas typically that direction would be negative
    // don't worry about that
    let (row, col) = (row as f32, col as f32);

    // slope to the top-left corner of the blocking square
    let top_left = col / (row + 1.0);
    // slope to the bottom-right corner of the blocking square
    let bottom_right = (col + 1.0) / (row);

    Shadow {
        start: top_left.into(),
        end: bottom_right.into(),
    }
}

/// Transforms a (row, col) pair from the given octant into an actual position.
/// Octants are labeled clockwise, sequentially
///
/// Note that (row, col) coordinates are such that the "row" axis extends positively
/// from the observer, and the "col" axis is seen as horizontal and varies from 0
/// to row (inclusive)
fn transform_octant(row: i32, col: i32, octant: usize) -> TilePos {
    let (x, y) = match octant {
        // 0 is the N/NE octant, and is a y-reflection of 3 (S/SE)
        0 => (col, -row),
        // 1 is the E/NE octant, and is an x/y reflection of 0 (N/NE)
        1 => (row, -col),
        // 2 is the E/SE octant, and is an x/y reflection of 3 (S/SE)
        2 => (row, col),
        // 3 is the S/SE octant, where the major axis is y (positive is South, as desired) and minor
        // is x, so line up with row,col perfectly
        3 => (col, row),
        // 4 is the S/SW octant, and is an x-reflection of 3 (S/SE)
        4 => (-col, row),
        // 5 is the W/SW octant, and is an x/y-reflection of 4 (S/SW)
        5 => (-row, col),
        // 6 is the W/NW octant, and is an x/y-reflection of 7 (N/NW)
        6 => (-row, -col),
        // 7 is the N/NW octant, and is an x-reflection of 0 (N/NE)
        7 => (-col, -row),
        // There are only octants 0..=7. The rusty thing would be to use enums,
        // but they're annoying for enumeration through, and the octant is only
        // used in this module, and only privately, so it should be alright
        _ => panic!("Unsupported octant: {}", octant),
    };

    TilePos { x, y }
}

pub fn refresh_visibility(observer_pos: TilePos, map: &mut Map, vis_range: i32) {
    if let Some(self_sq) = map.get_square_mut(observer_pos.x, observer_pos.y) {
        let can_see_self = can_see_through(*self_sq);
        self_sq.visibility = update_vis(self_sq.visibility, can_see_self);
    }

    for octant in 0..8 {
        refresh_octant_vis(observer_pos, octant, map, vis_range);
    }
}

fn update_vis(old: VisibilityType, can_see: bool) -> VisibilityType {
    if can_see {
        return VisibilityType::CurrentlyVisible;
    }

    match old {
        VisibilityType::CurrentlyVisible => VisibilityType::Remembered,
        VisibilityType::Remembered => VisibilityType::Remembered,
        VisibilityType::NotSeen => VisibilityType::NotSeen,
    }
}

fn set_visible(map: &mut Map, pos: TilePos, can_see: bool) {
    if let Some(square) = map.get_square_mut(pos.x, pos.y) {
        square.visibility = update_vis(square.visibility, can_see);
    }
}

//  TODO: vis_range MUST include the entire map! Or we will never set the edge of vision to be invisible
// in the case where you can no longer see something because it's just too far away
fn refresh_octant_vis(observer_pos: TilePos, octant: usize, map: &mut Map, vis_range: i32) {
    let mut line = ShadowLine::default();

    let mut full_shadow = false;

    for row in 1..vis_range {
        let mut all_occluded = true;

        for col in 0..=row {
            let pos = observer_pos + transform_octant(row, col, octant);

            if full_shadow {
                set_visible(map, pos, false);
                continue;
            }

            let projection = project_tile(row, col);

            if line.is_in_shadow(projection) {
                set_visible(map, pos, false);
                continue;
            }

            all_occluded = false;
            set_visible(map, pos, true);

            if !can_see_through(map.get_square(pos.x, pos.y)) {
                line.add_shadow(projection);
            }
        }

        if all_occluded {
            full_shadow = true;
        }
    }
}
