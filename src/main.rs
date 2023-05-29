use std::error::Error;

use cell_simulation::simulation::Simulation;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use cell_simulation::config::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Init SDL
    let sdl_ctx = sdl2::init()?;
    let mut event_pump = sdl_ctx.event_pump()?;

    // Create video context
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem
        .window(
            "Cell simulation",
            (SIMULATION_WIDTH * CELL_SIZE) as u32,
            (SIMULATION_HEIGHT * CELL_SIZE) as u32,
        )
        .position_centered()
        .build()?;
    // Create canvas, on which we will actually call draw-related functions, like `fill_rect()`
    let mut canvas = window.into_canvas().build()?;

    // Init the simulation itself
    let mut simulation = Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT);

    'main: loop {
        // Update logic
        simulation.update();

        // Handle SDL events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Render
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                let cell = simulation.cell_at(x, y).unwrap();

                if cell.empty {
                    continue;
                }

                let mut drawing_color = Color::RGB(70, 70, 70);
                if cell.alive {
                    drawing_color = cell.color;
                }

                canvas.set_draw_color(drawing_color);
                canvas.fill_rect(Rect::new(
                    (x * CELL_SIZE) as i32,
                    (y * CELL_SIZE) as i32,
                    CELL_SIZE as u32,
                    CELL_SIZE as u32,
                ))?;
            }
        }

        canvas.present();
    }

    Ok(())
}
