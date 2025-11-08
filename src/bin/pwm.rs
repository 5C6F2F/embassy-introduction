#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::OutputType,
    peripherals::TIM2,
    time::khz,
    timer::simple_pwm::{PwmPin, SimplePwm, SimplePwmChannel},
};
use embassy_time::Timer;

// Panic handler. Don't remove.
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let simple_pwm = SimplePwm::new(
        p.TIM2,
        Some(PwmPin::new(p.PA5, OutputType::PushPull)),
        None,
        None,
        None,
        khz(10),
        Default::default(),
    );

    let chs = simple_pwm.split();
    spawner.spawn(pwm(chs.ch1)).unwrap();
}

#[embassy_executor::task]
async fn pwm(mut ch: SimplePwmChannel<'static, TIM2>) {
    ch.enable();

    let max_duty_cycle = ch.max_duty_cycle();
    let step = (max_duty_cycle / 5) as usize;

    info!("PWM max duty {}", max_duty_cycle);

    loop {
        for duty_cycle in (0..=max_duty_cycle).step_by(step) {
            ch.set_duty_cycle(duty_cycle);
            info!("{}", ch.current_duty_cycle());
            Timer::after_millis(300).await;
        }
    }
}
