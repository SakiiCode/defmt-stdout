#![doc = include_str!("../README.md")]
use std::{
    io::{StdoutLock, Write, stdout},
    sync::Mutex,
};

use defmt::{Encoder, Logger};

#[defmt::global_logger]
struct StdLogger;

struct StdLockRef {
    lock: Option<StdoutLock<'static>>,
}

impl StdLockRef {
    pub const fn new() -> Self {
        Self { lock: None }
    }
}

unsafe impl Send for StdLockRef {}

static LOGGER: Mutex<StdLockRef> = Mutex::new(StdLockRef::new());
static ENCODER: Mutex<Encoder> = Mutex::new(Encoder::new());

unsafe impl Logger for StdLogger {
    fn acquire() {
        let mut logger = LOGGER.lock().expect("Logger mutex holder panicked");
        if logger.lock.is_some() {
            panic!("Stdout lock already acquired");
        }

        let mut lock = stdout().lock();

        ENCODER
            .lock()
            .expect("Encoder mutex holder panicked")
            .start_frame(write_callback(&mut lock));

        logger.lock = Some(lock);
    }
    unsafe fn flush() {
        LOGGER
            .lock()
            .ok()
            .and_then(|mut logger| logger.lock.as_mut()?.flush().ok());
    }
    unsafe fn release() {
        let mut logger = LOGGER.lock().expect("Logger mutex holder panicked");
        let mut lock = logger.lock.take().expect("Missing lock at release");

        ENCODER
            .lock()
            .expect("Encoder mutex holder panicked")
            .end_frame(write_callback(&mut lock));

        lock.flush().ok();
    }
    unsafe fn write(bytes: &[u8]) {
        let mut logger = LOGGER.lock().expect("Logger mutex holder panicked");
        let lock = logger.lock.as_mut().expect("Missing lock at write");

        ENCODER
            .lock()
            .expect("Encoder mutex holder panicked")
            .write(bytes, write_callback(lock));
    }
}

fn write_callback(lock: &mut StdoutLock<'static>) -> impl FnMut(&[u8]) {
    |bytes| {
        lock.write(bytes).expect("Could not write to stdout");
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_info() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--example", "basic", "-q"]);
        let stdout = cmd.unwrap().stdout;
        let output = String::from_utf8_lossy(&stdout).to_string();
        defmt::assert_eq!(output.as_str(), "INFO  This is an info\n");
    }
}
