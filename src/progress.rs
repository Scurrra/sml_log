#![allow(unused)]
use crate::logger::Logger;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::io::{self, Write};
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

pub struct Progress {
    logger: RefCell<Weak<Logger>>,
    unit: String,
    precision: u8,

    is_running: Cell<bool>,
    is_active: Cell<bool>,

    total: Cell<Option<u64>>,
    initial: Cell<Option<u64>>,
    current: Cell<Option<u64>>,

    initial_time: Cell<Option<Instant>>,
    current_time: Cell<Option<Instant>>,
    rate: Cell<Option<f64>>,

    old_length: Cell<Option<u64>>,
}

pub struct ProgressConfig {
    unit: String,
    precision: u8,
}

impl ProgressConfig {
    pub fn new(unit: String, precision: u8) -> Self {
        Self { unit, precision }
    }
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            unit: String::from("item"),
            precision: 3,
        }
    }
}

impl Progress {
    pub fn new(unit: String, precision: u8) -> Self {
        Self {
            logger: RefCell::new(Weak::new()),
            unit,
            precision,
            is_running: Cell::new(false),
            is_active: Cell::new(false),
            total: Cell::new(None),
            initial: Cell::new(None),
            current: Cell::new(None),
            initial_time: Cell::new(None),
            current_time: Cell::new(None),
            rate: Cell::new(None),
            old_length: Cell::new(None),
        }
    }

    pub fn from_config(config: ProgressConfig) -> Self {
        Self {
            logger: RefCell::new(Weak::new()),
            unit: config.unit.clone(),
            precision: config.precision,
            is_running: Cell::new(false),
            is_active: Cell::new(false),
            total: Cell::new(None),
            initial: Cell::new(None),
            current: Cell::new(None),
            initial_time: Cell::new(None),
            current_time: Cell::new(None),
            rate: Cell::new(None),
            old_length: Cell::new(None),
        }
    }

    pub fn with_logger(&self, logger: Weak<Logger>) {
        *self.logger.borrow_mut() = logger;
    }

    pub fn reset_logger(&self) {
        *self.logger.borrow_mut() = Weak::new();
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            logger: RefCell::new(Weak::new()),
            unit: String::from("item"),
            precision: 3,
            is_running: Cell::new(false),
            is_active: Cell::new(false),
            total: Cell::new(None),
            initial: Cell::new(None),
            current: Cell::new(None),
            initial_time: Cell::new(None),
            current_time: Cell::new(None),
            rate: Cell::new(None),
            old_length: Cell::new(None),
        }
    }
}

impl Progress {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_active.get() {
            return Err("Progress is not active".into());
        }

        if self.is_running.get() {
            return Err("Progress is already running".into());
        }
        self.is_running.set(true);

        if let Some(logger) = self.logger.borrow().upgrade() {
            self.old_length.set(Some(logger.log_progress()?));
        } else {
            // TODO: pretty print
            print!("");
            io::stdout().flush().unwrap();
        }

        Ok(())
    }
}
