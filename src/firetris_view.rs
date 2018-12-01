//! Firetris view.

use graphics::types::Color;
use graphics::{Context, Graphics, Transformed};

use firetris::{HEIGHT, WIDTH};
use FiretrisController;

/// Stores firetris view settings.
pub struct FiretrisViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of firetris along horizontal and vertical edge.
    pub size: [f64; 2],
    /// Background color.
    pub background_color: Color,
}

impl FiretrisViewSettings {
    /// Creates new firetris view settings.
    pub fn new() -> FiretrisViewSettings {
        FiretrisViewSettings {
            position: [100.0, 20.0],
            size: [128.0, 512.0],
            background_color: [0.1, 0.1, 0.5, 1.0],
        }
    }
}

/// Stores visual information about a firetris.
pub struct FiretrisView {
    /// Stores firetris view settings.
    pub settings: FiretrisViewSettings,
}

impl FiretrisView {
    /// Creates a new firetris view.
    pub fn new(settings: FiretrisViewSettings) -> FiretrisView {
        FiretrisView { settings: settings }
    }

    /// Draw firetris.
    pub fn draw<G: Graphics>(&self, controller: &FiretrisController, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        let firetris = &controller.firetris;
        let cellsize = self.settings.size[0] / WIDTH as f64;

        let ref settings = self.settings;
        let c = c.trans(settings.position[0], settings.position[1]);

        let border_rect = [
            -3.0,
            -3.0,
            cellsize * WIDTH as f64 + 6.0,
            cellsize * HEIGHT as f64 + 6.0,
        ];
        let board_rect = [0.0, 0.0, cellsize * WIDTH as f64, cellsize * HEIGHT as f64];

        // Draw board background.
        Rectangle::new([1.0; 4]).draw(border_rect, &c.draw_state, c.transform, g);
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw active piece.
        if let Some(ref piece) = firetris.active_piece {
            let x = piece.position[0];
            let y = piece.position[1];
            for block in piece.blocks.iter() {
                let block_rect = [
                    (block[0] + x) as f64 * cellsize + 0.5,
                    (block[1] + y) as f64 * cellsize + 0.5,
                    cellsize - 1.0,
                    cellsize - 1.0,
                ];
                Rectangle::new(piece.color).draw(block_rect, &c.draw_state, c.transform, g);
            }
        }

        // Draw settled blocks.
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                firetris.cells[row][col].as_ref().map(|color| {
                    let block_rect = [
                        col as f64 * cellsize + 0.5,
                        row as f64 * cellsize + 0.5,
                        cellsize - 1.0,
                        cellsize - 1.0,
                    ];
                    Rectangle::new(color.clone()).draw(block_rect, &c.draw_state, c.transform, g);
                });
            }
        }
    }
}
