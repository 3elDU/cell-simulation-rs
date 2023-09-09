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
                });
        });

        let map = handle.map();

        if is_mouse_button_pressed(MouseButton::Middle) {
            let (cx, cy) = mouse_position();
            let (x, y) = (cx as u16 / CELL_SIZE, cy as u16 / CELL_SIZE);
            println!(
                "{}",
                serde_json::to_string_pretty(map.get(x, y).unwrap()).unwrap()
            );
        }

        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                let cell = map.get(x, y).unwrap();

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
