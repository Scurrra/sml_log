#![allow(unused)]
use crate::progress::{Progress, ProgressConfig};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct Logger {
    pub progress: Rc<Progress>,

    quiet: bool,
    scope: String,
    prefix: String,
}

pub struct LoggerConfig {
    quiet: bool,
    scope: String,
}

impl LoggerConfig {
    pub fn new(scope: String, quiet: bool) -> Self {
        Self { quiet, scope }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            quiet: false,
            scope: String::new(),
        }
    }
}

impl Logger {
    pub fn new(scope: String, quiet: bool) -> Rc<Self> {
        let progress = Rc::new(Progress::default());
        let logger = Rc::new(Self {
            progress: Rc::clone(&progress),
            quiet,
            prefix: if scope.is_empty() {
                String::new()
            } else {
                format!("{}::", scope)
            },
            scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }

    pub fn from_config(config: LoggerConfig) -> Rc<Self> {
        let progress = Rc::new(Progress::default());
        let logger = Rc::new(Self {
            progress: Rc::clone(&progress),
            quiet: config.quiet,
            prefix: if config.scope.is_empty() {
                String::new()
            } else {
                format!("{}::", config.scope)
            },
            scope: config.scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }

    pub fn from_configs(logger_config: LoggerConfig, progress_config: ProgressConfig) -> Rc<Self> {
        let progress = Rc::new(Progress::from_config(progress_config));
        let logger = Rc::new(Self {
            progress: Rc::clone(&progress),
            quiet: logger_config.quiet,
            prefix: if logger_config.scope.is_empty() {
                String::new()
            } else {
                format!("{}::", logger_config.scope)
            },
            scope: logger_config.scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }
}

impl Logger {
    pub fn log_progress(&self) -> Result<u64, Box<dyn Error>> {
        Ok(0)
    }
}
