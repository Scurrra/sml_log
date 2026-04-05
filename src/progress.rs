use crate::logger::Logger;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

pub struct Progress {
    logger: RefCell<Weak<Logger>>,
    unit: String,
    precision: u8,

    is_running: bool,
    is_active: bool,

    total: Option<u64>,
    initial: Option<u64>,
    current: Option<u64>,

    initial_time: Option<Instant>,
    current_time: Option<Instant>,
    rate: Option<f64>,

    old_length: Option<u64>,
}

pub struct ProgressConfig {
    unit: String,
    precision: u8,
}

impl ProgressConfig {
    pub fn new(unit: String, precision: u8) -> Self {
        Self { unit, precision }
    }

    pub fn default() -> Self {
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
            is_running: false,
            is_active: false,
            total: None,
            initial: None,
            current: None,
            initial_time: None,
            current_time: None,
            rate: None,
            old_length: None,
        }
    }

    pub fn default() -> Self {
        Self {
            logger: RefCell::new(Weak::new()),
            unit: String::from("item"),
            precision: 3,
            is_running: false,
            is_active: false,
            total: None,
            initial: None,
            current: None,
            initial_time: None,
            current_time: None,
            rate: None,
            old_length: None,
        }
    }

    pub fn from_config(config: ProgressConfig) -> Self {
        Self {
            logger: RefCell::new(Weak::new()),
            unit: config.unit.clone(),
            precision: config.precision,
            is_running: false,
            is_active: false,
            total: None,
            initial: None,
            current: None,
            initial_time: None,
            current_time: None,
            rate: None,
            old_length: None,
        }
    }

    pub fn with_logger(&self, logger: Weak<Logger>) {
        *self.logger.borrow_mut() = logger;
    }

    pub fn reset_logger(&self) {
        *self.logger.borrow_mut() = Weak::new();
    }
}
