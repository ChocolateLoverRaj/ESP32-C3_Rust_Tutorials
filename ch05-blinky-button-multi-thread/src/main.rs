#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_idf_hal::{gpio, prelude::*};
use statig::{prelude::*, InitializedStatemachine};
use std::{sync::mpsc::*, thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio8.into_output().unwrap();
    let btn = peripherals.pins.gpio6.into_input().unwrap();

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
    print_type_of(&tx);

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm, rx))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn, tx))
        .unwrap();
}

fn blinky_fsm_thread(mut fsm: InitializedStatemachine<led_fsm::Blinky>, rx: Receiver<bool>) {
    let mut led_count = 0;
    loop {
        led_count += 1;
        if led_count > 10 {
            led_count = 0;
            fsm.handle(&led_fsm::Event::TimerElapsed);
        }
        match rx.try_recv() {
            Ok(_) => fsm.handle(&led_fsm::Event::ButtonPressed),
            Err(_) => {}
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn button_thread(btn: gpio::Gpio6<gpio::Input>, tx: Sender<bool>) {
    let mut btn_state = true;
    loop {
        if btn.is_high().unwrap() {
            if !btn_state {
                btn_state = true;
                tx.send(btn_state).unwrap();
            }
        } else {
            btn_state = false;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
