use egui::DragValue;
use macroquad::prelude::*;
use mimalloc::MiMalloc;

use cell_simulation::config::*;
use cell_simulation::simulation::Simulation;

// Use MiMalloc as a global allocator
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
    let mut handles = [
        Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT).spawn_thread(),
        Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT).spawn_thread(),
        Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT).spawn_thread(),
        Simulation::new(SIMULATION_WIDTH, SIMULATION_HEIGHT).spawn_thread(),
    ];
    let mut selected = 0;
    let mut pasted_bot_json = String::new();
    let mut pasted_bot_x: u16 = 0;
    let mut pasted_bot_y: u16 = 0;
    let mut limit_tps = false;
    let mut tps_limit = 25usize;

    loop {
        let handle = &mut handles[selected];
        // try to refresh the data for selected simulation
        let _ = handle.try_refresh();

        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Simulation selector")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut selected, 0, "1");
                        ui.radio_value(&mut selected, 1, "2");
                        ui.radio_value(&mut selected, 2, "3");
                        ui.radio_value(&mut selected, 3, "4");
                    })
                });

            egui::Window::new("Selected simulation")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.button(if handle.is_paused() {
                            "Unpause"
                        } else {
                            "Pause"
                        })
                        .clicked()
                        .then(|| handle.toggle_pause());

                        ui.button("Reset map").clicked().then(|| handle.reset_map());
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("TPS: {}", handle.fps()));
                        ui.label(format!("Iterations: {}", handle.iterations()))
                    });

                    ui.vertical(|ui| {
                        ui.checkbox(&mut limit_tps, "Limit TPS").changed().then(|| {
                            handle.set_target_tps(if limit_tps { Some(tps_limit) } else { None });
                        });
                        ui.add(egui::Slider::new(&mut tps_limit, 1..=100))
                            .changed()
                            .then(|| {
                                handle.set_target_tps(if limit_tps {
                                    Some(tps_limit)
                                } else {
                                    None
                                });
                            });
                    });
                });

            if let Some(selected_cell) = handle.selected_cell() {
                egui::Window::new("Selected cell")
                    .resizable(false)
                    .default_open(false)
                    .show(ctx, |ui| {
                        ui.label(format!(
                            "Coordinates: {}, {}",
                            selected_cell.x(),
                            selected_cell.y()
                        ));
                    });
            }

            egui::Window::new("Paste from JSON")
                .resizable(false)
                .default_open(false)
                .show(ctx, |ui| {
                    ui.label("Paste JSON object here");
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.text_edit_multiline(&mut pasted_bot_json);
                        });

                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(DragValue::new(&mut pasted_bot_x).clamp_range(0..=SIMULATION_WIDTH));
                        ui.label("Y:");
                        ui.add(
                            DragValue::new(&mut pasted_bot_y).clamp_range(0..=SIMULATION_HEIGHT),
                        );

                        ui.button("Paste!").clicked().then(|| {
                            handle.set_cell(
                                pasted_bot_x,
                                pasted_bot_y,
                                serde_json::from_str(&pasted_bot_json).unwrap(),
                            );
                        });
                    });
                });
        });

        if is_mouse_button_pressed(MouseButton::Middle) {
            let (cx, cy) = mouse_position();
            let (x, y) = (cx as u16 / CELL_SIZE, cy as u16 / CELL_SIZE);
            println!(
                "{}",
                serde_json::to_string_pretty(handle.map().get(x, y).unwrap()).unwrap()
            );
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            let (cx, cy) = mouse_position();
            let (x, y) = (cx as u16 / CELL_SIZE, cy as u16 / CELL_SIZE);
            pasted_bot_x = x;
            pasted_bot_y = y;
            handle.select_cell(x, y);
        }

        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                let cell = handle.map().get(x, y).unwrap();

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
