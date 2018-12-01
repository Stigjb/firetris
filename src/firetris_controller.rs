//! Firetris controller.

use piston::input::GenericEvent;
use rand;

use firetris::{Firetris, FiretrisPiece, PieceType, HEIGHT, WIDTH};

/// Handles events for Sudoku game.
pub struct FiretrisController {
    /// Stores the firetris state.
    pub firetris: Firetris,
    since_last_update: f64,
}

impl FiretrisController {
    /// Creates a new firetris controller.
    pub fn new(firetris: Firetris) -> FiretrisController {
        FiretrisController {
            firetris: firetris,
            since_last_update: 0.0,
        }
    }

    pub fn collision(&self, next: &FiretrisPiece) -> bool {
        let cells = self.firetris.cells;
        let pos = next.position;
        for block in next.blocks.iter() {
            let row = block[1] + pos[1];
            let col = block[0] + pos[0];
            if col < 0 || col >= WIDTH as i8 {
                return true;
            }
            if row < 0 || row >= HEIGHT as i8 {
                return true;
            }
            if cells[row as usize][col as usize].is_some() {
                return true;
            }
        }
        false
    }

    /// Handles events;
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key};

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Return {
                let piece_type = rand::random::<PieceType>();
                self.firetris.active_piece = Some(FiretrisPiece::from(piece_type));
                return;
            };
            if self.firetris.active_piece.is_none() {
                return;
            };
            let possible_next = self.firetris.active_piece.as_ref().map(|piece| match key {
                Key::Up => piece.rotate(),
                Key::Down => piece.drop(),
                Key::Right => piece.right(),
                Key::Left => piece.left(),
                Key::Space => {
                    let mut next = piece.drop();
                    let mut nextnext = next.drop();
                    while !self.collision(&nextnext) {
                        next = nextnext;
                        nextnext = next.drop();
                    }
                    next
                }
                _ => piece.clone(),
            });
            possible_next.map(|p| {
                if !self.collision(&p) {
                    self.firetris.active_piece = Some(p);
                }
            });
        }

        if let Some(args) = e.update_args() {
            self.since_last_update += args.dt;
            if self.since_last_update >= 0.5 {
                self.since_last_update -= 0.5;
                let maybe_next = self.firetris.active_piece.as_ref().map(FiretrisPiece::drop);
                match maybe_next.as_ref().map(|p| self.collision(&p)) {
                    Some(true) => {
                        self.firetris.settle();
                        let piece_type = rand::random::<PieceType>();
                        self.firetris.active_piece = Some(FiretrisPiece::from(piece_type));
                    }
                    Some(false) => self.firetris.active_piece = maybe_next,
                    None => {}
                }
            }
        }
    }
}
