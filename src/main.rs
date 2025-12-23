use defmt_stdout as _;

fn main(){
    defmt::println!("This is a println");
    defmt::info!("This is an info");
    defmt::warn!("This is a warning");
    defmt::error!("This is an error");
}