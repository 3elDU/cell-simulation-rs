pub mod renderer;
pub mod runner;
pub mod simulation;

use egui::DragValue;
use egui::Slider;
use macroquad::prelude::*;

use renderer::RenderingMode;
use runner::SimulationRunner;
use simulation::config::*;
use simulation::Simulation;

fn window_config() -> Conf {
    let default_config = Config::default();

    Conf {
        window_title: "Cell simulation".to_string(),
        window_width: (default_config.width * default_config.cell_size) as i32,
        window_height: (default_config.height * default_config.cell_size) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // Start 4 simulations, each in it's own thread
    let mut simulation = SimulationRunner::start_new(Simulation::new(None));
    let mut rendering_mode = RenderingMode::Normal;

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

            egui::Window::new("Settings")
                .resizable(false)
                .show(ctx, |ui| {
                    let mut config = *simulation.config();

                    ui.horizontal(|ui| {
                        ui.label("Mutation percent");
                        ui.add(Slider::new(&mut config.mutation_percent, 0.0..=100.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Start energy");
                        ui.add(
                            DragValue::new(&mut config.start_energy)
                                .clamp_range(0.0..=f32::INFINITY)
                                .speed(0.1),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Required energy for reproduction");
                        ui.add(DragValue::new(&mut config.reproduction_required_energy));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Max age");
                        ui.add(DragValue::new(&mut config.cell_max_age));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Photosynthesis energy");
                        ui.add(DragValue::new(&mut config.photosynthesis_energy).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Attack energy");
                        ui.add(DragValue::new(&mut config.attack_energy).speed(0.05));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Movement cost");
                        ui.add(DragValue::new(&mut config.movement_cost).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Noop cost");
                        ui.add(DragValue::new(&mut config.noop_cost));
                    });

                    if config != *simulation.config() {
                        simulation
                            .update_config(config)
                            .expect("Failed to update the simulation configuration");
                    }
                });

            egui::Window::new("Rendering mode")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.radio_value(&mut rendering_mode, RenderingMode::Normal, "Normal");
                    ui.radio_value(&mut rendering_mode, RenderingMode::Energy, "Energy");
                    ui.radio_value(&mut rendering_mode, RenderingMode::Lifetime, "Lifetime");
                });
        });

        let config = simulation.config();
        for x in 0..config.width {
            for y in 0..config.height {
                let cell = simulation.map().get(x, y).unwrap();

                if cell.empty {
                    continue;
                }

                let color = if cell.alive {
                    rendering_mode.render(cell, config).into()
                } else {
                    Color::from_rgba(100, 100, 100, 255)
                };

                draw_rectangle(
                    (x * config.cell_size) as f32,
                    (y * config.cell_size) as f32,
                    config.cell_size as f32,
                    config.cell_size as f32,
                    color,
                );
            }
        }
        egui_macroquad::draw();

        next_frame().await;
    }
}
