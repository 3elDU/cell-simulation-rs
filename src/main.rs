pub mod config;
pub mod runner;
pub mod simulation;

use macroquad::prelude::*;

use config::*;
use runner::SimulationRunner;
use simulation::Simulation;

fn window_config() -> Conf {
    Conf {
        window_title: "Cell simulation".to_string(),
        window_width: (SIMULATION_WIDTH * CELL_SIZE) as i32,
        window_height: (SIMULATION_HEIGHT * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // Start 4 simulations, each in it's own thread
    let mut simulation =
        SimulationRunner::start_new(Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT));

    loop {
        simulation.update();

        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Simulation controls")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.button(if simulation.is_paused() {
                            "Unpause"
                        } else {
                            "Pause"
                        })
                        .clicked()
                        .then(|| simulation.toggle_pause());

                        ui.button("Reset map").clicked().then(|| simulation.reset());
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("TPS: {}", simulation.tps()));
                        ui.label(format!("Iterations: {}", simulation.iterations()))
                    });
                });
        });

        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                let cell = simulation.map().get(x, y).unwrap();

                if cell.empty {
                    continue;
                }

                let color = if cell.alive {
                    cell.color.into()
                } else {
                    Color::from_rgba(100, 100, 100, 255)
                };

                draw_rectangle(
                    (x * CELL_SIZE) as f32,
                    (y * CELL_SIZE) as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    color,
                );
            }
        }
        egui_macroquad::draw();

        next_frame().await;
    }
}
