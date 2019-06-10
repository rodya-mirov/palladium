mod params;
mod rand_gen;

pub use params::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Square {
    Void, // off the map
    Open,
    Floor,
    Wall,
    VerticalDoor,
    HorizontalDoor,
}

impl Square {
    pub fn to_char(self) -> char {
        match self {
            Square::Void => '*',
            Square::Open => '*',
            Square::Floor => ' ',
            Square::Wall => '█',
            Square::VerticalDoor => 'd',
            Square::HorizontalDoor => 'd',
        }
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
            return Square::Void;
        }

        self.get_index(x as usize, y as usize)
            .map(|ind| self.cells[ind])
            .unwrap_or(Square::Void)
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
