# esp32c3_rust_tutorials

This will be a series of embedded Rust tutorials.  I'm making this repo as a way to record everything I learn so I don't forget it and hopefully it'll help other people.  Embedded Rust is mature enough for a general embedded user, but there's not as many examples out there to follow.


## Steps to set up a new ESP32 Rust project
1. [Install](https://github.com/esp-rs/rust-build) Rust and Xtensa build tools
    - Make sure to `sudo chmod +x export-esp.sh`
2. Start a project using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) from the private repo home `dir`. I chose all the default options
```
# STD Project
cargo generate https://github.com/esp-rs/esp-idf-template cargo
# NO-STD (Bare-metal) Project
cargo generate https://github.com/esp-rs/esp-template
```
3. Build the `Hello World` program by running `cargo build` in the new project dir. This will take a while to build the first time:
```
cd esp32-rust
cargo build
...
Finished dev [optimized + debuginfo] target(s) in 6m 40s
```
4. Flash the ESP32 with the build artifact:
```
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/project
```
5. Connect to ESP32 and monitor
```
espmonitor /dev/ttyACM0
```

## Other
Pull in code for submodules with:
```
git submodule update --init --recursive
```

## Chapter 1 - Blinky
Classic blinky program where we'll dig into the basics of initializing a pin and looping while blining on/off

## Chapter 2 - Blinky thread
We'll take our blinky example and move it into a thread.  

## Chapter 3 - Blinky FSM
Next we'll put the blinky logic into a FSM using the [statig crate](https://github.com/mdeloof/statig).

## Chapter 4 - Blinky + Button HSM
Add a button to the blinky program and make a hierarchical state machine using the [statig crate](https://github.com/mdeloof/statig).

## Chapter 5 - Blinky + Button Multi-thread
Move the button logic into another thread and send a message to the blinky thread when the button is pressed

## Chapter 6 - Blinky + Button Multi-thread with MPMC
Take the logic from chapter 5 and add another thread which will require use `multi-producer multi-consumer` crate called [crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

## Chapter 8 - Blinky + WiFi
Blinky, but now we're connected to WiFi

## Roadmap:
- Logging (https://github.com/knurling-rs/defmt)
- Debug project (https://github.com/knurling-rs/probe-run)
- MQTT transfer
- Pub/sub (https://github.com/jakmeier/nuts)
- Timer to generate blinky
- ADC-to-pwm blinky
- DMA
- SPI
- I2C connect to GPIO expander
- CLI
