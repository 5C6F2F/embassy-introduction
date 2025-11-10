#![no_std]
#![no_main]

use core::sync::atomic::{AtomicI32, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    Peri,
    peripherals::{TIM1, TIM2},
    timer::{
        GeneralInstance4Channel, TimerPin,
        qei::{self, Qei, QeiPin},
    },
};
use embassy_time::Timer;

// Panic handler. Don't remove.
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let encoder1 = Encoder::new(p.TIM1, p.PA8, p.PA9, 2048, RotateDirection::Forward);
    let encoder2 = Encoder::new(p.TIM2, p.PA0, p.PA1, 2048, RotateDirection::Forward);

    static ENCODER1_COUNT: AtomicI32 = AtomicI32::new(0);
    static ENCODER2_COUNT: AtomicI32 = AtomicI32::new(0);

    spawner
        .spawn(update_encoder_tim1(encoder1, &ENCODER1_COUNT))
        .unwrap();
    spawner.spawn(print_encoder_count(&ENCODER1_COUNT)).unwrap();

    spawner
        .spawn(update_encoder_tim2(encoder2, &ENCODER2_COUNT))
        .unwrap();
    spawner.spawn(print_encoder_count(&ENCODER2_COUNT)).unwrap();
}

#[embassy_executor::task]
async fn update_encoder_tim1(encoder: Encoder<'static, TIM1>, count: &'static AtomicI32) {
    update_encoder(encoder, count).await;
}

#[embassy_executor::task]
async fn update_encoder_tim2(encoder: Encoder<'static, TIM2>, count: &'static AtomicI32) {
    update_encoder(encoder, count).await;
}

async fn update_encoder(
    mut encoder: Encoder<'static, impl GeneralInstance4Channel>,
    count: &'static AtomicI32,
) {
    loop {
        encoder.update();
        count.store(encoder.get_count(), Ordering::Relaxed);
        Timer::after_millis(5).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn print_encoder_count(count: &'static AtomicI32) {
    loop {
        info!("{}", count.load(Ordering::Relaxed));
        Timer::after_millis(500).await;
    }
}

#[allow(unused)]
enum RotateDirection {
    Forward,
    Reverse,
}

struct Encoder<'d, T: GeneralInstance4Channel> {
    qei: Qei<'d, T>,
    qei_resolution: u32,
    direction: RotateDirection,
    last_hw_count: u16,
    software_count: i32,
}

#[allow(dead_code)]
impl<'d, T: GeneralInstance4Channel> Encoder<'d, T> {
    fn new(
        tim: Peri<'d, T>,
        phase_a_pin: Peri<'d, impl TimerPin<T, qei::Ch1>>,
        phase_b_pin: Peri<'d, impl TimerPin<T, qei::Ch2>>,
        ppr: u32,
        direction: RotateDirection,
    ) -> Self {
        let phase_a_pin = QeiPin::new(phase_a_pin);
        let phase_b_pin = QeiPin::new(phase_b_pin);

        // QEIではA相とB相の立ち上がり/立ち下がりを用いて4逓倍でカウントされる
        let qei_resolution = ppr * 4;

        let qei = Qei::new(tim, phase_a_pin, phase_b_pin);

        // 現在のハードウェアカウントを読み取り、初期値とする
        let last_hw_count = qei.count();

        Self {
            qei,
            qei_resolution,
            direction,
            last_hw_count,
            software_count: 0,
        }
    }

    fn get_count(&self) -> i32 {
        self.software_count
    }

    /// 現在の回転数を取得
    pub fn get_rotations(&self) -> f32 {
        self.get_count() as f32 / self.qei_resolution as f32
    }

    /// ハードウェアカウントを読み取り、ソフトウェアカウントを更新します。
    ///
    /// ## [重要]
    ///
    /// このメソッドは、エンコーダーがハードウェアカウント上限の半分
    /// （32,767カウント ≒ 分解能2048で4回転）
    /// 回転するよりも短い周期で、外部の制御ループから定期的に呼び出す必要があります。
    fn update(&mut self) {
        let current_hw_count = self.qei.count();
        let delta = self.calculate_delta(current_hw_count, self.last_hw_count);

        match self.direction {
            RotateDirection::Forward => self.software_count += delta,
            RotateDirection::Reverse => self.software_count -= delta,
        }

        self.last_hw_count = current_hw_count;
    }

    /// オーバーフローを考慮してエンコーダーのカウント変化を計算
    fn calculate_delta(&self, current_count: u16, last_count: u16) -> i32 {
        if current_count > last_count {
            let delta = current_count - last_count;
            if delta <= 32767 {
                // 100 -> 200 : +100
                // 増加としてカウント
                delta as i32
            } else {
                // 50 -> 65486 : -100
                // 減少としてカウント

                // -(65536 - delta)と等しい
                -(delta.wrapping_neg() as i32)
            }
        } else {
            let delta = last_count - current_count;
            if delta <= 32767 {
                // 200 -> 100 : -100
                // 減少としてカウント
                -(delta as i32)
            } else {
                // 65486 -> 50 : +100
                // 増加としてカウント

                // 65536 - deltaと等しい
                delta.wrapping_neg() as i32
            }
        }
    }
}
