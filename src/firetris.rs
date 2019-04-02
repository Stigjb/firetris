//! Game board logic.

use graphics::types::Color;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::mem;

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
pub struct Piece {
    pub color: Color,
    pub blocks: [[i8; 2]; 4],
    pub position: [i8; 2],
}

impl Piece {
    pub fn from(name: &PieceType) -> Self {
        match *name {
            PieceType::T => Self {
                color: [1.0, 0.0, 0.0, 1.0],
                blocks: [[-1, 0], [0, 0], [1, 0], [0, 1]],
                position: [4, 0],
            },
            PieceType::Block => Self {
                color: [0.0, 1.0, 0.0, 1.0],
                blocks: [[0, 0], [0, 1], [1, 0], [1, 1]],
                position: [4, 0],
            },
            PieceType::Straight => Self {
                color: [0.5, 0.3, 1.0, 1.0],
                blocks: [[-1, 0], [0, 0], [1, 0], [2, 0]],
                position: [4, 0],
            },
            PieceType::L => Self {
                color: [1.0, 1.0, 0.0, 1.0],
                blocks: [[0, -1], [0, 0], [0, 1], [1, 1]],
                position: [4, 1],
            },
            PieceType::RevL => Self {
                color: [0.0, 1.0, 1.0, 1.0],
                blocks: [[0, -1], [0, 0], [0, 1], [-1, 1]],
                position: [4, 1],
            },
            PieceType::S => Self {
                color: [1.0, 0.0, 1.0, 1.0],
                blocks: [[0, 0], [1, 0], [-1, 1], [0, 1]],
                position: [4, 0],
            },
            PieceType::Z => Self {
                color: [1.0, 0.5, 0.0, 1.0],
                blocks: [[-1, 0], [0, 0], [0, 1], [1, 1]],
                position: [4, 0],
            },
        }
    }

    pub fn drop(&self) -> Self {
        Self {
            position: [self.position[0], self.position[1] + 1],
            ..*self
        }
    }

    pub fn left(&self) -> Self {
        Self {
            position: [self.position[0] - 1, self.position[1]],
            ..*self
        }
    }

    pub fn right(&self) -> Self {
        Self {
            position: [self.position[0] + 1, self.position[1]],
            ..*self
        }
    }

    pub fn rotate(&self) -> Self {
        let mut new_blocks = Vec::new();
        for block in &self.blocks {
            let new_block = [block[1], -block[0]];
            new_blocks.push(new_block);
        }
        let mut new_blocks_array = [[0; 2]; 4];
        new_blocks_array.copy_from_slice(&new_blocks[..]);
        Self {
            blocks: new_blocks_array,
            ..*self
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
    pub active_piece: Option<Piece>,
    pub stored_piece: Option<Piece>,
}

impl Firetris {
    /// Creates a new game board.
    pub fn new() -> Self {
        Self {
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
        for block in &piece.blocks {
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
        let score = match rows.len() {
            1 => 50,
            2 => 120,
            3 => 180,
            4 => 250,
            n => n,  // Should realistically only be zero
        };
        self.score += score;
        for row in rows {
            let mut last_row = [None; WIDTH];
            for i in 0..=row {
                mem::swap(&mut self.cells[i], &mut last_row)
            }
        }
    }
}
