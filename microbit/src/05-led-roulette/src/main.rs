#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, prelude::*}
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let delay = 30;

    let mut what_to_show = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0]
    ];

    loop {
        for i in 0..9 {
            if i == 0 {
                for j in 0..5 {
                    clearArray(&mut what_to_show);
                    what_to_show[i][j] = 1;
                    display.show(&mut timer, what_to_show, delay);
                }
            } else if i == 4 {
                for j in (0..5).rev() {
                    clearArray(&mut what_to_show);
                    what_to_show[i][j] = 1;
                    display.show(&mut timer, what_to_show, delay);
                }
            } else if i < 5 {
                clearArray(&mut what_to_show);
                what_to_show[i][4] = 1;
                display.show(&mut timer, what_to_show, delay);
            } else {
                clearArray(&mut what_to_show);
                what_to_show[4 - (i - 5)][0] = 1;
                display.show(&mut timer, what_to_show, delay);
            }
        }
    }
}

fn clearArray(arr: &mut [[u8 ; 5] ; 5]) {
    *arr = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0]
    ];
}
