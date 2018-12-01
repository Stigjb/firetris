//! Game board logic.

use graphics::types::Color;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub enum PieceType {
    T,
    Straight,
    L,
    RevL,
    Block,
    S,
    Z,
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0, 7) {
            0 => PieceType::T,
            1 => PieceType::Straight,
            2 => PieceType::L,
            3 => PieceType::RevL,
            4 => PieceType::Block,
            5 => PieceType::S,
            _ => PieceType::Z,
        }
    }
}

#[derive(Clone)]
pub struct FiretrisPiece {
    pub color: Color,
    pub blocks: [[i8; 2]; 4],
    pub position: [i8; 2],
}

impl FiretrisPiece {
    pub fn from(name: PieceType) -> FiretrisPiece {
        match name {
            PieceType::T => FiretrisPiece {
                color: [1.0, 0.0, 0.0, 1.0],
                blocks: [[-1, 0], [0, 0], [1, 0], [0, 1]],
                position: [4, 0],
            },
            PieceType::Block => FiretrisPiece {
                color: [0.0, 1.0, 0.0, 1.0],
                blocks: [[0, 0], [0, 1], [1, 0], [1, 1]],
                position: [4, 0],
            },
            PieceType::Straight => FiretrisPiece {
                color: [0.5, 0.3, 1.0, 1.0],
                blocks: [[-1, 0], [0, 0], [1, 0], [2, 0]],
                position: [4, 0],
            },
            PieceType::L => FiretrisPiece {
                color: [1.0, 1.0, 0.0, 1.0],
                blocks: [[0, -1], [0, 0], [0, 1], [1, 1]],
                position: [4, 1],
            },
            PieceType::RevL => FiretrisPiece {
                color: [0.0, 1.0, 1.0, 1.0],
                blocks: [[0, -1], [0, 0], [0, 1], [-1, 1]],
                position: [4, 1],
            },
            PieceType::S => FiretrisPiece {
                color: [1.0, 0.0, 1.0, 1.0],
                blocks: [[0, 0], [1, 0], [-1, 1], [0, 1]],
                position: [4, 0],
            },
            PieceType::Z => FiretrisPiece {
                color: [1.0, 0.5, 0.0, 1.0],
                blocks: [[-1, 0], [0, 0], [0, 1], [1, 1]],
                position: [4, 0],
            },
        }
    }

    pub fn drop(&self) -> FiretrisPiece {
        FiretrisPiece {
            color: self.color,
            blocks: self.blocks,
            position: [self.position[0], self.position[1] + 1],
        }
    }

    pub fn left(&self) -> FiretrisPiece {
        FiretrisPiece {
            color: self.color,
            blocks: self.blocks,
            position: [self.position[0] - 1, self.position[1]],
        }
    }

    pub fn right(&self) -> FiretrisPiece {
        FiretrisPiece {
            color: self.color,
            blocks: self.blocks,
            position: [self.position[0] + 1, self.position[1]],
        }
    }

    pub fn rotate(&self) -> FiretrisPiece {
        let mut new_blocks = Vec::new();
        for block in self.blocks.iter() {
            let new_block = [block[1], -block[0]];
            new_blocks.push(new_block);
        }
        let mut new_blocks_array = [[0; 2]; 4];
        new_blocks_array.copy_from_slice(&new_blocks[..]);
        FiretrisPiece {
            color: self.color,
            blocks: new_blocks_array,
            position: self.position,
        }
    }
}

/// Stores game board information.
pub struct Firetris {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[Option<Color>; WIDTH]; HEIGHT],
    pub score: usize,
    pub level: u8,
    pub active_piece: Option<FiretrisPiece>,
    pub stored_piece: Option<FiretrisPiece>,
}

impl Firetris {
    /// Creates a new game board.
    pub fn new() -> Firetris {
        Firetris {
            cells: [[None; WIDTH]; HEIGHT],
            score: 0,
            level: 1,
            active_piece: None,
            stored_piece: None,
        }
    }

    pub fn settle(&mut self) {
        let piece = self.active_piece.clone().unwrap();
        let x = piece.position[0];
        let y = piece.position[1];
        for block in piece.blocks.iter() {
            let x = (x + block[0]) as usize;
            let y = (y + block[1]) as usize;
            self.cells[y][x] = Some(piece.color);
        }
        self.check_filled_lines();
    }

    fn check_filled_lines(&mut self) {
        let rows = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_i, r)| r.iter().all(|v| v.is_some()))
            .map(|(i, _r)| i)
            .collect::<Vec<usize>>();
        for row in rows {
            let mut last_row = [None; WIDTH];
            for i in 0..=row {
                let new_last_row = self.cells[i].clone();
                self.cells[i] = last_row;
                last_row = new_last_row;
            }
        }
    }
}
