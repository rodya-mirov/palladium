mod params;
mod rand_gen;

pub use params::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Square {
    Open,
    Floor,
    Wall,
    VerticalDoor,
    HorizontalDoor,
}

impl Square {
    fn to_char(&self) -> char {
        match self {
            Square::Open => '*',
            Square::Floor => ' ',
            Square::Wall => '█',
            Square::VerticalDoor => '¤',
            Square::HorizontalDoor => '¤',
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
    pub fn get_square(&self, x: usize, y: usize) -> Result<&Square, ()> {
        let ind = self.get_index(x, y)?;

        Ok(&self.cells[ind])
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

    pub fn draw(&self) {
        for (ind, square) in self.cells.iter().enumerate() {
            if ind % self.width == 0 {
                println!("");
            }
            print!("{}", square.to_char());
        }
        println!("");
    }

    pub fn make_random(params: &MapGenerationParams) -> Map {
        rand_gen::rand_gen(params)
    }
}
