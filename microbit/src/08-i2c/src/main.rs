#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use core::str::from_utf8;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use heapless::Vec;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, Measurement};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(lsm303agr::MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "Error reading command\r\n").unwrap();
                nb::block!(serial.flush()).unwrap();
            }

            if byte == 13 {
                break;
            }
        }

        if from_utf8(&buffer).unwrap() == "accelerometer\r" {
            write!(serial, "You typed accelerometer\r\n").unwrap();
            let data = sensor.accel_data().unwrap();
            write!(serial, "x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else if from_utf8(&buffer).unwrap() == "magnetometer\r" {
            write!(serial, "You typed magnetometer\r\n").unwrap();
            let data = sensor.mag_data().unwrap();
            write!(serial, "x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else {
            write!(serial, "Command not found\r\n").unwrap();
        }
    }
}

