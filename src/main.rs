#![no_std]
#![no_main]

use esp_backtrace as _;
use log::*;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    esp_println::logger::init_logger(log::LevelFilter::Info);
    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Initializing PSRAM allocator...");
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);
    info!("Done"); // never printed on this board

    loop {}
}
