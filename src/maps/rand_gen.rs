#![allow(deprecated)]
// use deprecated RNG because the rand team screwed up, the non-deprecated thing is broken
// that's what you get with pre-1.0 software i guess

use super::*;

use std::cmp::min;

use rand::{Isaac64Rng, Rng, SeedableRng};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Room {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Room {
    fn bad_touch(&self, other: &Room) -> bool {
        fn dimension_bad(a_min: usize, a_max: usize, b_min: usize, b_max: usize) -> bool {
            if a_min == b_max || a_max == b_min {
                false
            } else if a_max + 1 < b_min || b_max + 1 < a_min {
                false
            } else {
                true
            }
        }

        dimension_bad(self.left, self.right, other.left, other.right) && dimension_bad(self.top, self.bottom, other.top, other.bottom)
    }
}

pub fn rand_gen(params: &MapGenerationParams) -> Map {
    let width = params.map_dimensions.map_width;
    let height = params.map_dimensions.map_height;
    let seed = params.seed;

    let capacity = width * height;
    let mut cells = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        cells.push(Square::OPEN);
    }

    let mut map = Map { width, height, cells };

    let min_width = params.room_dimensions.room_min_width;
    let max_width = params.room_dimensions.room_max_width;

    let min_height = params.room_dimensions.room_min_height;
    let max_height = params.room_dimensions.room_max_height;

    let mut rng = Isaac64Rng::seed_from_u64(seed);

    for x in 0..width {
        map.set_square(x, 0, Square::WALL).expect("Indices should be valid");
        map.set_square(x, height - 1, Square::WALL).expect("Indices should be valid");
    }

    for y in 0..height {
        map.set_square(0, y, Square::WALL).expect("Indices should be valid");
        map.set_square(width - 1, y, Square::WALL).expect("Indices should be valid");
    }

    let mut rooms: Vec<Room> = Vec::new();
    let mut retries_remaining = params.max_retries;

    'room_loop: while retries_remaining > 0 {
        let room_x = rng.gen_range(0, width - min_width);
        let room_width = rng.gen_range(min_width, min(max_width, width - room_x));

        let room_y = rng.gen_range(0, height - min_height);
        let room_height = rng.gen_range(min_height, min(max_height, height - room_y));

        let room = Room {
            left: room_x,
            right: room_x + room_width - 1,
            top: room_y,
            bottom: room_y + room_height - 1,
        };

        for old_room in &rooms {
            if old_room.bad_touch(&room) {
                retries_remaining -= 1;
                continue 'room_loop;
            }
        }

        for x in room.left..(room.right + 1) {
            map.set_square(x, room.top, Square::WALL).expect("Indices should be valid");
            map.set_square(x, room.bottom, Square::WALL).expect("Indices should be valid");
        }

        for y in room.top..(room.bottom + 1) {
            map.set_square(room.left, y, Square::WALL).expect("Indices should be valid");
            map.set_square(room.right, y, Square::WALL).expect("Indices should be valid");
        }

        for x in (room.left + 1)..room.right {
            for y in (room.top + 1)..room.bottom {
                map.set_square(x, y, Square::FLOOR).expect("Indices should be valid");
            }
        }

        rooms.push(room);
    }

    if retries_remaining == 0 {
        println!("Got {} rooms :shrug:", rooms.len());
    }

    map
}
