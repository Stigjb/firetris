//! Firetris view.

use graphics::color::WHITE;
use graphics::types::Color;
use graphics::{Context, Graphics, Transformed};

use firetris::{HEIGHT, WIDTH};
use FiretrisController;

/// Stores firetris view settings.
pub struct Settings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of firetris along horizontal and vertical edge.
    pub size: [f64; 2],
    /// Background color.
    pub background_color: Color,
}

impl Settings {
    /// Creates new firetris view settings.
    pub fn new() -> Self {
        Self {
            position: [100.0, 20.0],
            size: [128.0, 512.0],
            background_color: [0.1, 0.1, 0.5, 1.0],
        }
    }
}

/// Stores visual information about a firetris.
pub struct View {
    /// Stores firetris view settings.
    pub settings: Settings,
}

impl View {
    /// Creates a new firetris view.
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Draw firetris.
    pub fn draw<G: Graphics>(&self, controller: &FiretrisController, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        let firetris = &controller.firetris;
        let cellsize = self.settings.size[0] / WIDTH as f64;

        let settings = &self.settings;
        let c = c.trans(settings.position[0], settings.position[1]);

        let border_rect = [
            -3.0,
            -3.0,
            cellsize * WIDTH as f64 + 6.0,
            cellsize * HEIGHT as f64 + 6.0,
        ];
        let board_rect = [0.0, 0.0, cellsize * WIDTH as f64, cellsize * HEIGHT as f64];

        // Draw board background.
        Rectangle::new(WHITE).draw(border_rect, &c.draw_state, c.transform, g);
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw active piece.
        if let Some(ref piece) = firetris.active_piece {
            for block in &piece.blocks {
                let block_rect = get_block_rect(
                    (block[1] + piece.position[1]) as usize,
                    (block[0] + piece.position[0]) as usize,
                    cellsize,
                );
                Rectangle::new(piece.color).draw(block_rect, &c.draw_state, c.transform, g);
            }
        }

        // Draw settled blocks.
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if let Some(color) = firetris.cells[row][col].as_ref() {
                    let block_rect = get_block_rect(row, col, cellsize);
                    Rectangle::new(*color).draw(block_rect, &c.draw_state, c.transform, g);
                };
            }
        }
    }
}

fn get_block_rect(row: usize, col: usize, cellsize: f64) -> [f64; 4] {
    [
        col as f64 * cellsize + 0.5,
        row as f64 * cellsize + 0.5,
        cellsize - 1.0,
        cellsize - 1.0,
    ]
}
