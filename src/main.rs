#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use embassy_nrf;
use embassy_executor::Spawner;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Hello, world!");
}

