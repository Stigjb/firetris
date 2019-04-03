extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub use firetris::Firetris;
pub use firetris_controller::FiretrisController;
pub use firetris_view::{Settings, View};

mod firetris;
mod firetris_controller;
mod firetris_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("FireTris", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
    let mut gl = GlGraphics::new(opengl);

    let firetris = Firetris::new();
    let mut firetris_controller = FiretrisController::new(firetris);
    let firetris_view_settings = Settings::new();
    let firetris_view = View::new(firetris_view_settings);

    while let Some(e) = events.next(&mut window) {
        firetris_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([0.3, 0.3, 0.3, 1.0], g);
                firetris_view.draw(&firetris_controller, &c, g);
            });
        }
    }
}
