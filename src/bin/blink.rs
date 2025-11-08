#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

// Panic handler. Don't remove.
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // マイコン初期化
    let p = embassy_stm32::init(Default::default());

    // LED用ピンの設定（PA5を出力モードで使用）
    let mut led = Output::new(p.PA5, Level::Low, Speed::Low);
    let duration = Duration::from_millis(500);

    loop {
        info!("LED ON");
        led.set_high();
        Timer::after_millis(duration.as_millis()).await;

        info!("LED OFF");
        led.set_low();
        Timer::after_millis(duration.as_millis()).await;
    }
}
