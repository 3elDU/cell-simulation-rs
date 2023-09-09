use crate::map::Map;

use super::bot;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub struct Simulation {
    width: u16,
    height: u16,
    iterations: usize,
    paused: bool,
    tps: usize,
    prev_tps_check: Instant,
    /// Keep the number of iterations from previous TPS check to be able to compare
    /// it to the current number of iterations, thus, calculating frames per second.
    prev_iterations: usize,
    map: Map,
    /// Keeping the simulation data inside Arc, so we don't have to copy the whole structure
    /// each time we want to send it through channel, only clone the pointer.
    /// Update the data only when it was successfully sent through the channel.
    simulation_data_cache: Arc<SimulationData>,
}

impl Simulation {
    /// Create a new simulation with map of given width and height.
    /// Also calls `generate_map()` automatically.
    pub fn new(width: u16, height: u16) -> Self {
        Simulation {
            width,
            height,
            paused: true,
            iterations: 0,
            tps: 0,
            prev_tps_check: Instant::now(),
            prev_iterations: 0,
            map: Map::new(width, height),
            simulation_data_cache: Arc::new(SimulationData {
                iterations: 0,
                paused: true,
                fps: 0,
                map: Map::new(width, height),
            }),
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

        let handle = SimulationThreadHandle {
            data: Arc::clone(&self.simulation_data_cache),
            rx,
            map_reset_trigger: map_reset_trigger_tx,
            pause_trigger: pause_trigger_tx,
        };

        thread::spawn(move || loop {
            if let Ok(()) = pause_trigger.try_recv() {
                self.paused = !self.paused;
            }
            if let Ok(()) = map_reset_trigger.try_recv() {
                self.iterations = 0;
                self.prev_iterations = 0;
                self.map.generate_map();
            }

            self.update();

            if tx.try_send(Arc::clone(&self.simulation_data_cache)).is_ok() {
                // Refresh the data only if the structure was sent through a channel
                self.simulation_data_cache = Arc::new(SimulationData {
                    iterations: self.iterations,
                    fps: self.tps,
                    paused: self.paused,
                    map: self.map.clone(),
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
                        bot::Bot::new_empty(orig_pos.0, orig_pos.1),
                    );
                }

                self.map.set(bot.x(), bot.y(), bot);
            }
        }

        self.iterations += 1;
        self.measure_fps();
    }
}

/// A structure representing simulation data, which is passed between threads
struct SimulationData {
    iterations: usize,
    paused: bool,
    fps: usize,
    map: Map,
}

/// A handle to a thread running the simulation, via which the simulation can be controlled
pub struct SimulationThreadHandle {
    data: Arc<SimulationData>,
    rx: Receiver<Arc<SimulationData>>,
    pause_trigger: Sender<()>,
    map_reset_trigger: Sender<()>,
}

impl SimulationThreadHandle {
    /// Try to receive new simulation data from the thread
    pub fn try_refresh(&mut self) -> Result<(), TryRecvError> {
        match self.rx.try_recv() {
            Ok(data) => {
                self.data = data;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn fps(&self) -> usize {
        self.data.fps
    }

    pub fn iterations(&self) -> usize {
        self.data.iterations
    }

    pub fn is_paused(&self) -> bool {
        self.data.paused
    }
    pub fn toggle_pause(&self) {
        self.pause_trigger.send(()).unwrap();
    }

    pub fn map(&mut self) -> &Map {
        &self.data.map
    }
    pub fn reset_map(&mut self) {
        self.map_reset_trigger.send(()).unwrap();
    }
}
