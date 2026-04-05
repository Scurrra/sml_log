use crate::progress::{Progress, ProgressConfig};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Logger {
    progress: RefCell<Rc<Progress>>,

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

    pub fn default() -> Self {
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
            progress: RefCell::new(Rc::clone(&progress)),
            quiet,
            prefix: if scope.len() == 0 {
                scope.clone()
            } else {
                scope.clone() + "::"
            },
            scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }

    pub fn from_config(config: LoggerConfig) -> Rc<Self> {
        let progress = Rc::new(Progress::default());
        let logger = Rc::new(Self {
            progress: RefCell::new(Rc::clone(&progress)),
            quiet: config.quiet,
            prefix: if config.scope.len() == 0 {
                config.scope.clone()
            } else {
                config.scope.clone() + "::"
            },
            scope: config.scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }

    pub fn from_configs(logger_config: LoggerConfig, progress_config: ProgressConfig) -> Rc<Self> {
        let progress = Rc::new(Progress::from_config(progress_config));
        let logger = Rc::new(Self {
            progress: RefCell::new(Rc::clone(&progress)),
            quiet: logger_config.quiet,
            prefix: if logger_config.scope.len() == 0 {
                logger_config.scope.clone()
            } else {
                logger_config.scope.clone() + "::"
            },
            scope: logger_config.scope,
        });
        progress.with_logger(Rc::downgrade(&logger));

        logger
    }
}
