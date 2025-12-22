use std::{io::{self, StdoutLock, Write}, sync::Mutex};

#[defmt::global_logger]
struct StdLogger;

struct StdLoggerRef{
    lock:Option<StdoutLock<'static>>
}

impl StdLoggerRef {
    pub const fn new()->Self{
        Self{lock:None}
    }
}

unsafe impl Send for StdLoggerRef{}

static LOGGER: Mutex<StdLoggerRef> = Mutex::new(StdLoggerRef::new());

unsafe impl defmt::Logger for StdLogger {
    fn acquire() {
        let mut logger = LOGGER.lock().expect("Mutex holder panicked");
        match logger.lock {
            Some(_) => panic!("Stdout lock already acquired"),
            None => logger.lock = Some(io::stdout().lock())
        };
        
    }
    unsafe fn flush() {
        let mut logger = LOGGER.lock().expect("Mutex holder panicked");
        match &mut logger.lock {
            Some(lock) => lock.flush().expect("Could not flush stdout"),
            None => panic!("Stdout lock was not acquired")
        };
    }
    unsafe fn release() {
        let mut logger = LOGGER.lock().expect("Mutex holder panicked");
        logger.lock.take();
    }
    unsafe fn write(bytes: &[u8]) {
        let mut logger = LOGGER.lock().expect("Mutex holder panicked");
        match &mut logger.lock {
            Some(lock) => lock.write(bytes).expect("Could not write to stdout"),
            None => panic!("Stdout lock was not acquired")
        };
    }
}
