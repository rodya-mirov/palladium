use super::*;

use crate::rng::{make_rng, PalladRng, Rng};
use std::cmp::{max, min};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Room {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Door {
    x: usize,
    y: usize,
}

impl Room {
    fn bad_touch(&self, other: &Room) -> bool {
        fn dimension_bad(a_min: usize, a_max: usize, b_min: usize, b_max: usize) -> bool {
            !((a_min == b_max || a_max == b_min) || (a_max + 1 < b_min || b_max + 1 < a_min))
        }

        dimension_bad(self.left, self.right, other.left, other.right) && dimension_bad(self.top, self.bottom, other.top, other.bottom)
    }

    fn try_make_door(&self, other: &Room, rng: &mut PalladRng) -> Option<Door> {
        // horizontal touching, maybe
        if self.left == other.right || self.right == other.left {
            let y_min = max(self.top + 1, other.top + 1);
            let y_max = min(self.bottom - 1, other.bottom - 1);

            if y_min > y_max {
                None
            } else {
                let x = if self.left == other.right { self.left } else { self.right };
                let y = rng.gen_range(y_min, y_max + 1);
                Some(Door { x, y })
            }
        } else if self.top == other.bottom || self.bottom == other.top {
            let x_min = max(self.left + 1, other.left + 1);
            let x_max = min(self.right - 1, other.right - 1);

            if x_min > x_max {
                None
            } else {
                let x = rng.gen_range(x_min, x_max + 1);
                let y = if self.top == other.bottom { self.top } else { self.bottom };
                Some(Door { x, y })
            }
        } else {
            None
        }
    }
}

fn make_random_rooms(params: &MapGenerationParams, rng: &mut PalladRng) -> Vec<Room> {
    let width = params.map_dimensions.map_width;
    let height = params.map_dimensions.map_height;

    let min_width = params.room_dimensions.room_min_width;
    let max_width = params.room_dimensions.room_max_width;

    let min_height = params.room_dimensions.room_min_height;
    let max_height = params.room_dimensions.room_max_height;

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

        rooms.push(room);
    }

    rooms
}

fn make_raw_square(square_type: SquareType) -> Square {
    Square { square_type }
}

pub struct MapGenResult {
    pub width: usize,
    pub height: usize,
    // cells is row-by-row (C-indexed) for cells[x,y] is cells[y * width + x]
    pub cells: Vec<Square>,
    // just an array of random stuff that could be generated
    pub others: Vec<GeneratedEntity>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Square {
    pub square_type: SquareType,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GeneratedEntity {
    Rubbish(TilePos),
    Pillar(TilePos),
}

impl MapGenResult {
    fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!(
                "MapGen: Invalid x/y: x: {}, y: {}, width: {}, height: {}",
                x, y, self.width, self.height
            );
        }

        x + y * self.width
    }

    fn set_square(&mut self, x: usize, y: usize, square: Square) {
        let ind = self.index(x, y);
        self.cells[ind] = square;
    }
}

pub fn rand_gen(params: &MapGenerationParams) -> MapGenResult {
    let open = make_raw_square(SquareType::Open);
    let floor = make_raw_square(SquareType::Floor);
    let wall = make_raw_square(SquareType::Wall);
    let door = make_raw_square(SquareType::Door);

    let width = params.map_dimensions.map_width;
    let height = params.map_dimensions.map_height;
    let seed = params.seed;

    let capacity = width * height;
    let mut cells = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        cells.push(open);
    }

    let mut map = MapGenResult {
        width,
        height,
        cells,
        others: Vec::new(),
    };

    for x in 0..width {
        map.set_square(x, 0, wall);
        map.set_square(x, height - 1, wall);
    }

    for y in 0..height {
        map.set_square(0, y, wall);
        map.set_square(width - 1, y, wall);
    }

    let mut rng = make_rng(seed);

    let rooms = make_random_rooms(params, &mut rng);

    for room in &rooms {
        for x in room.left..=room.right {
            map.set_square(x, room.top, wall);
            map.set_square(x, room.bottom, wall);
        }

        for y in room.top..=room.bottom {
            map.set_square(room.left, y, wall);
            map.set_square(room.right, y, wall);
        }

        for x in (room.left + 1)..room.right {
            for y in (room.top + 1)..room.bottom {
                let next_perc = rng.gen_range(1, 101); // 1 to 100
                if next_perc <= 5 {
                    map.others.push(GeneratedEntity::Rubbish(TilePos { x: x as i32, y: y as i32 }));
                } else if next_perc <= 10 {
                    map.others.push(GeneratedEntity::Pillar(TilePos { x: x as i32, y: y as i32 }));
                };
                map.set_square(x, y, floor);
            }
        }
    }

    let num_rooms = rooms.len();

    for i in 0..num_rooms {
        let a = rooms[i];
        for j in i + 1..num_rooms {
            let b = rooms[j];

            if let Some(door_val) = a.try_make_door(&b, &mut rng) {
                map.set_square(door_val.x, door_val.y, door);
            }
        }
    }

    map
}
