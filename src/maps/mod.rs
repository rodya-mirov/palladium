mod params;
mod rand_gen;

pub use params::*;

const UNSEEN_VOID_SQUARE: Square = Square {
    square_type: SquareType::Void,
    visibility: VisibilityType::NotSeen,
};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Square {
    pub square_type: SquareType,
    pub visibility: VisibilityType,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum VisibilityType {
    NotSeen,
    Remembered,
    CurrentlyVisible,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SquareType {
    Void, // off the map
    Open, // not allocated to anything (should not be accessible to the player)
    Floor,
    Wall,
    Door,    // can walk, can't see through
    Rubbish, // can't walk, can see through
    Pillar,  // can't walk, can't see through
}

impl Square {
    pub fn to_char(self) -> char {
        match self.square_type {
            SquareType::Void => '*',
            SquareType::Open => '*',
            SquareType::Floor => ' ',
            SquareType::Wall => '█',
            SquareType::Door => 'd',
            SquareType::Rubbish => '`',
            SquareType::Pillar => 'I',
        }
    }
}

fn make_unseen_square(square_type: SquareType) -> Square {
    Square {
        square_type,
        visibility: VisibilityType::NotSeen,
    }
}

pub struct Map {
    width: usize,
    height: usize,
    // cells is row-by-row (C-indexed) for cells[x,y] is cells[y * width + x]
    cells: Vec<Square>,
}

impl Map {
    pub fn get_square(&self, x: i32, y: i32) -> Square {
        if x < 0 || y < 0 {
            return UNSEEN_VOID_SQUARE;
        }

        self.get_index(x as usize, y as usize)
            .map(|ind| self.cells[ind])
            .unwrap_or(UNSEEN_VOID_SQUARE)
    }

    pub fn get_square_mut(&mut self, x: i32, y: i32) -> Option<&mut Square> {
        if x < 0 || y < 0 {
            return None;
        }

        match self.get_index(x as usize, y as usize) {
            Ok(ind) => self.cells.get_mut(ind),
            Err(()) => None,
        }
    }

    pub fn set_square(&mut self, x: usize, y: usize, square: Square) -> Result<(), ()> {
        let ind = self.get_index(x, y)?;

        self.cells[ind] = square;

        Ok(())
    }

    fn get_index(&self, x: usize, y: usize) -> Result<usize, ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }

        let max = self.width * self.height;

        let ind = self.width * y + x;
        if ind >= max {
            return Err(());
        }

        Ok(ind)
    }

    pub fn make_random(params: &MapGenerationParams) -> Map {
        rand_gen::rand_gen(params)
    }
}
