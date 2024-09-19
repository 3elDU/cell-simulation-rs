pub mod bot;
pub mod color;
pub mod direction;
pub mod gene;
pub mod map;

use bot::Bot;
use map::Map;
use rand::prelude::*;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub struct Simulation {
    width: usize,
    height: usize,
    iterations: usize,
    paused: bool,
    tps: usize,
    prev_tps_check: Instant,
    /// Keep the number of iterations from previous TPS check to be able to compare
    /// it to the current number of iterations, thus, calculating frames per second.
    prev_iterations: usize,
    map: Map<Bot>,

    selected_bot_coordinates: Option<(usize, usize)>,
    // Keep a copy of the bot even if it no longer exists on the map
    selected_bot: Option<Bot>,

    /// Keeping the simulation data inside Arc, so we don't have to copy the whole structure
    /// each time we want to send it through channel, only clone the pointer.
    /// Update the data only when it was successfully sent through the channel.
    simulation_data_cache: Arc<SimulationData>,

    // Simulation will try to run at this exact TPS, if possible
    target_tps: Option<usize>,
}

impl Simulation {
    /// Create a new simulation with map of given width and height.
    /// Also calls `generate_map()` automatically.
    pub fn new(width: usize, height: usize) -> Self {
        let mut simulation = Simulation {
            width,
            height,
            paused: true,
            iterations: 0,
            tps: 0,
            prev_tps_check: Instant::now(),
            prev_iterations: 0,
            map: Map::new(width, height),
            selected_bot_coordinates: None,
            selected_bot: None,
            simulation_data_cache: Arc::new(SimulationData {
                iterations: 0,
                paused: true,
                fps: 0,
                map: Map::new(width, height),
                selected_bot: None,
                target_tps: None,
            }),
            target_tps: None,
        };

        simulation.generate_map();
        simulation
    }

    pub fn generate_map(&mut self) {
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                // 20% chance to generate an alive bot
                let cell_is_alive = rng.gen_bool(1.0 / 5.0);

                let bot = if cell_is_alive {
                    Bot::new_random(x, y)
                } else {
                    Bot::new_empty(x, y)
                };

                self.map.set(x, y, bot);
            }
        }
    }

    /// Spawns a thread running the simulation,
    /// and returns a handle via which the simulation data can be accessed.
    /// This method consumes the object, as the simulation will be moved to separate thread
    pub fn spawn_thread(mut self) -> SimulationThreadHandle {
        // Channel with simulation data
        let (tx, rx) = mpsc::sync_channel(1);
        // Pause / map reset trigger
        let (pause_trigger_tx, pause_trigger) = mpsc::channel();
        let (map_reset_trigger_tx, map_reset_trigger) = mpsc::channel();
        let (set_bot_trigger_tx, set_bot_trigger) = mpsc::channel();
        let (select_bot_tx, select_bot_trigger) = mpsc::channel();
        let (target_tps_tx, target_tps_trigger) = mpsc::channel();

        let handle = SimulationThreadHandle {
            data: Arc::clone(&self.simulation_data_cache),
            rx,
            map_reset_trigger: map_reset_trigger_tx,
            pause_trigger: pause_trigger_tx,
            set_bot_trigger: set_bot_trigger_tx,
            select_cell_trigger: select_bot_tx,
            target_tps_trigger: target_tps_tx,
        };

        thread::spawn(move || loop {
            if let Ok(()) = pause_trigger.try_recv() {
                self.paused = !self.paused;
            }
            if let Ok(()) = map_reset_trigger.try_recv() {
                self.iterations = 0;
                self.prev_iterations = 0;
                self.generate_map();
            }
            if let Ok((x, y, mut bot)) = set_bot_trigger.try_recv() {
                bot.set_coordinates(x, y);
                *self.map.get_mut(x, y).unwrap() = bot;
            }
            if let Ok(selection) = select_bot_trigger.try_recv() {
                self.selected_bot_coordinates = selection;
            }
            if let Ok(target_tps) = target_tps_trigger.try_recv() {
                self.target_tps = target_tps
            }

            let start = Instant::now();

            self.update();
            self.measure_fps();

            if let Some(target_tps) = self.target_tps {
                let to_sleep = 1.0 / target_tps as f64 - start.elapsed().as_secs_f64();
                if to_sleep > 0.0 {
                    thread::sleep(Duration::from_secs_f64(to_sleep));
                }
            }

            if tx.try_send(Arc::clone(&self.simulation_data_cache)).is_ok() {
                // Refresh the data only if the structure was sent through a channel
                self.simulation_data_cache = Arc::new(SimulationData {
                    iterations: self.iterations,
                    fps: self.tps,
                    paused: self.paused,
                    map: self.map.clone(),
                    selected_bot: self.selected_bot,
                    target_tps: self.target_tps,
                });
            }
        });

        handle
    }

    fn measure_fps(&mut self) {
        let now = Instant::now();
        // Measure FPS with intervals of 1 second
        if now.duration_since(self.prev_tps_check).as_secs() > 0 {
            self.tps = self.iterations - self.prev_iterations;

            self.prev_iterations = self.iterations;
            self.prev_tps_check = now;
        }
    }

    /// Updates the simulation
    pub fn update(&mut self) {
        if self.paused {
            // If paused, sleep for 10ms to avoid wasting CPU cycles
            thread::sleep(Duration::from_millis(10));
            return;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let mut bot = *self.map.get(x, y).unwrap();
                let orig_pos = bot.coordinates();

                bot.update(&mut self.map);

                // if bot position was changed, set empty cell at previous position
                if orig_pos != bot.coordinates() {
                    self.map.set(
                        orig_pos.0,
                        orig_pos.1,
                        Bot::new_empty(orig_pos.0, orig_pos.1),
                    );
                }

                // Update coordinates of the selected bot
                if let Some(selected_bot_coordinates) = self.selected_bot_coordinates {
                    if selected_bot_coordinates == orig_pos {
                        self.selected_bot_coordinates = Some(bot.coordinates());
                        self.selected_bot = Some(bot);
                    }
                }

                self.map.set(bot.x(), bot.y(), bot);
            }
        }

        self.iterations += 1;
    }
}

/// A structure representing simulation data, which is passed between threads
struct SimulationData {
    iterations: usize,
    paused: bool,
    fps: usize,
    map: Map<Bot>,
    selected_bot: Option<Bot>,
    target_tps: Option<usize>,
}

/// A handle to a thread running the simulation, via which the simulation can be controlled
pub struct SimulationThreadHandle {
    data: Arc<SimulationData>,
    rx: Receiver<Arc<SimulationData>>,
    pause_trigger: Sender<()>,
    map_reset_trigger: Sender<()>,
    set_bot_trigger: Sender<(usize, usize, Bot)>,
    select_cell_trigger: Sender<Option<(usize, usize)>>,
    target_tps_trigger: Sender<Option<usize>>,
}

impl SimulationThreadHandle {
    /// Try to receive new simulation data from the thread
    pub fn try_refresh(&mut self) -> Result<(), TryRecvError> {
        self.data = self.rx.try_recv()?;
        Ok(())
    }

    #[inline(always)]
    pub fn fps(&self) -> usize {
        self.data.fps
    }

    #[inline(always)]
    pub fn iterations(&self) -> usize {
        self.data.iterations
    }

    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        self.data.paused
    }
    pub fn toggle_pause(&self) {
        self.pause_trigger.send(()).unwrap();
    }

    #[inline(always)]
    pub fn map(&self) -> &Map<Bot> {
        &self.data.map
    }
    pub fn reset_map(&mut self) {
        self.map_reset_trigger.send(()).unwrap();
    }

    pub fn set_cell(&mut self, x: usize, y: usize, bot: Bot) {
        self.set_bot_trigger.send((x, y, bot)).unwrap();
    }

    pub fn select_cell(&mut self, x: usize, y: usize) {
        self.select_cell_trigger.send(Some((x, y))).unwrap();
    }
    #[inline(always)]
    pub fn selected_cell(&self) -> Option<Bot> {
        self.data.selected_bot
    }

    #[inline(always)]
    pub fn target_tps(&self) -> Option<usize> {
        self.data.target_tps
    }
    pub fn set_target_tps(&mut self, target: Option<usize>) {
        self.target_tps_trigger.send(target).unwrap();
    }
}
