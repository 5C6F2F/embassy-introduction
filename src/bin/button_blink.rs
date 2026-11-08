#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    exti::{self, ExtiInput},
    gpio::{Level, Output, Pull, Speed},
    interrupt,
    mode::Async,
};
use embassy_time::Timer;

// Panic handler. Don't remove.
use {defmt_rtt as _, panic_probe as _};

// EXTI10〜15の割り込みをEmbassyの割り込みハンドラに紐付ける
bind_interrupts!(struct Irqs {
    EXTI15_10 => exti::InterruptHandler<interrupt::typelevel::EXTI15_10>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let led = Output::new(p.PA5, Level::Low, Speed::Low);
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up, Irqs);
    spawner.spawn(button_blink(led, button).unwrap());

    loop {
        println!("Hello, world!");
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn button_blink(mut led: Output<'static>, mut button: ExtiInput<'static, Async>) {
    loop {
        button.wait_for_falling_edge().await;
        led.set_high();
        info!("led turned on");

        button.wait_for_rising_edge().await;
        led.set_low();
        info!("led turned off");
    }
}
