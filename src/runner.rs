use std::{
    sync::{
        mpsc::{self, Receiver, SendError, Sender, SyncSender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use crate::simulation::{bot::Bot, map::Map, Simulation};

pub enum SimulationCommand {
    TogglePause,
    Reset,
    SelectCell(usize, usize),
}

#[derive(Clone, Default)]
pub struct SimulationMetadata {
    iterations: usize,
    tps: usize,
    paused: bool,
    map: Map<Bot>,
    selected_bot: Option<Bot>,
}

/// This structure is a handle to the [`SimulationRunner`].
/// There is no new method, as the way to get a [`SimulationHandle`] is through [`SimulationRunner::start_new`]
pub struct SimulationHandle {
    tx: Sender<SimulationCommand>,
    rx: Receiver<Arc<SimulationMetadata>>,

    metadata: Arc<SimulationMetadata>,
}

/// This structure contains the actual simulation object,
/// and gets transferred to another thread
pub struct SimulationRunner {
    /// The type here is [`SyncSender`], with the capacity of 1, so that we do not send anything,
    /// until the main thread consumes the previous metadata sent
    tx: SyncSender<Arc<SimulationMetadata>>,
    rx: Receiver<SimulationCommand>,

    /// Metadata is stored in the variable to not compute it each iteration,
    /// and is revalidated only when sent successfully.
    /// Wrapped in [`Arc`], because [`SyncSender::try_send`] consumes the variable sent.
    next_metadata: Arc<SimulationMetadata>,

    paused: bool,

    /// Measuring Ticks Per Second works by storing current amount of iterations in `previous_iterations`,
    /// and after a second, subtract `previous_iterations` from current `iterations`. This way
    /// we get how many iterations happened in a second. This process repeats indefinetely, measuring
    /// TPS each second
    tps: usize,
    previous_iterations: usize,
    previous_tps_check: Instant,

    simulation: Simulation,
}

impl SimulationRunner {
    /// Returns a handle to the thread, [`SimulationHandle`]
    pub fn start_new(simulation: Simulation) -> SimulationHandle {
        let (metadata_tx, metadata_rx) = mpsc::sync_channel(1);
        let (command_tx, command_rx) = mpsc::channel();

        let mut runner = Self {
            rx: command_rx,
            tx: metadata_tx,
            next_metadata: Arc::new(SimulationMetadata::default()),
            paused: true,
            tps: 0,
            previous_iterations: 0,
            previous_tps_check: Instant::now(),
            simulation,
        };

        runner.construct_metadata();
        let metadata = runner.next_metadata.clone();

        thread::spawn(move || runner.run());

        SimulationHandle {
            tx: command_tx,
            rx: metadata_rx,
            metadata,
        }
    }

    fn handle_commands(&mut self) {
        if let Ok(command) = self.rx.try_recv() {
            match command {
                SimulationCommand::TogglePause => self.paused = !self.paused,
                SimulationCommand::Reset => self.simulation.reset(),
                SimulationCommand::SelectCell(x, y) => {
                    let _ = self.simulation.select_bot(x, y);
                }
            }
        }
    }
    fn send_metadata(&mut self) {
        if let Ok(()) = self.tx.try_send(self.next_metadata.clone()) {
            // Compute the next metadata
            self.construct_metadata();
        }
    }
    fn construct_metadata(&mut self) {
        self.next_metadata = Arc::new(SimulationMetadata {
            iterations: self.simulation.iterations(),
            tps: self.tps,
            paused: self.paused,
            map: self.simulation.map().clone(),
            selected_bot: self.simulation.selected_bot(),
        });
    }

    fn measure_tps(&mut self) {
        if self.previous_tps_check.elapsed().as_millis() > 1000 {
            self.tps = self.simulation.iterations() - self.previous_iterations;
            self.previous_iterations = self.simulation.iterations();
            self.previous_tps_check = Instant::now();
        }
    }

    fn run(mut self) {
        loop {
            self.handle_commands();

            if !self.paused {
                self.simulation.update();
                self.measure_tps();
            } else {
                // Sleep for 10ms when paused, to not waste clock cycles
                thread::sleep(Duration::from_millis(10));
            }

            self.send_metadata();
        }
    }
}

impl SimulationHandle {
    pub fn reset(&mut self) -> Result<(), SendError<SimulationCommand>> {
        self.tx.send(SimulationCommand::Reset)
    }

    pub fn toggle_pause(&mut self) -> Result<(), SendError<SimulationCommand>> {
        self.tx.send(SimulationCommand::TogglePause)
    }
    pub fn is_paused(&self) -> bool {
        self.metadata.paused
    }
    pub fn iterations(&self) -> usize {
        self.metadata.iterations
    }
    pub fn tps(&self) -> usize {
        self.metadata.tps
    }

    pub fn map(&self) -> &Map<Bot> {
        &self.metadata.map
    }

    pub fn select_bot(&mut self, x: usize, y: usize) -> Result<(), SendError<SimulationCommand>> {
        self.tx.send(SimulationCommand::SelectCell(x, y))
    }
    pub fn selected_bot(&self) -> Option<&Bot> {
        self.metadata.selected_bot.as_ref()
    }

    // Receive metadata update from the thread
    pub fn update(&mut self) {
        if let Ok(metadata) = self.rx.try_recv() {
            self.metadata = metadata;
        }
    }
}
