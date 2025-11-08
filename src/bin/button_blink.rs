#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
};
use embassy_time::Timer;

// Panic handler. Don't remove.
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let led = Output::new(p.PA5, Level::Low, Speed::Low);
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);
    spawner.spawn(button_blink(led, button)).unwrap();

    loop {
        println!("Hello, world!");
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn button_blink(mut led: Output<'static>, mut button: ExtiInput<'static>) {
    loop {
        button.wait_for_falling_edge().await;
        led.set_high();
        info!("led turned on");

        button.wait_for_rising_edge().await;
        led.set_low();
        info!("led turned off");
    }
}
