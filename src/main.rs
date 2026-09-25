//! PWM example for the RP Pico2.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_15, PWM_SLICE7};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Timer;
use panic_probe as _;

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"PWM Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico2 PWM output on gpio 15"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main(executor = "embassy_rp::executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(pwm(p.PWM_SLICE7, p.PIN_15).unwrap());
}

#[embassy_executor::task]
async fn pwm(slice7: Peri<'static, PWM_SLICE7>, pin15: Peri<'static, PIN_15>) {
    let desired_freq_hz: u32 = 25_000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;

    // Panic if divider is not large enough to make top fit into u16,
    // or if the desired frequency is too high for the given clock and divider.
    let top: u16 = (clock_freq_hz / (desired_freq_hz * divider as u32))
        .checked_sub(1)
        .and_then(|t| t.try_into().ok())
        .expect("PWM frequency out of range for this divider");

    let mut c = Config::default();
    c.top = top;
    c.divider = divider.into();

    debug!("PWM configured with top: {}", top);
    debug!("PWM configured with divider: {}", divider);

    let mut pwm = Pwm::new_output_b(slice7, pin15, c);

    loop {
        pwm.set_duty_cycle(0).unwrap();
        Timer::after_secs(1).await;
        pwm.set_duty_cycle_percent(25).unwrap();
        Timer::after_secs(1).await;
    }

}
