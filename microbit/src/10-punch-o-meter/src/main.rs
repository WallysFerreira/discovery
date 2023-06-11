#![deny(unsafe_code)]
#![no_main]
#![no_std]
use cortex_m_rt::entry;
use nb::Result;
use rtt_target::{ rtt_init_print, rprintln };
use panic_rtt_target as _; 
use lsm303agr::{AccelOutputDataRate, AccelScale, Lsm303agr};
use microbit::{ hal::Timer, hal::prelude::*, hal::twim, pac::twim0::frequency::FREQUENCY_A, };

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    let threshold = 200;
    let mut timer = Timer::new(board.TIMER0); 
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    loop {
        let mut max = 0;

        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            if data.x > threshold {
                timer.start(1_000_000_u32);
                while timer.wait() != Ok(()) {
                    if data.x > max {
                        max = data.x;
                    }
                }
                rprintln!("Seu maximo foi {}", max);
            }

        }
    }
}
