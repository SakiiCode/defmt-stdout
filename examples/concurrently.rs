use defmt_stdout as _;

fn main() {
    std::thread::spawn(|| {
        loop {
            defmt::info!("This is an info");
        }
    });

    std::thread::spawn(|| {
        loop {
            defmt::warn!("This is a warning");
        }
    });
    loop {}
}
