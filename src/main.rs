//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod gc;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::PinState;
use gc::*;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut data_write_pin = pins.gpio17.into_push_pull_output_in_state(PinState::Low);
    let mut data_read_pin = pins.gpio16.into_floating_input();
    delay.delay_ms(2000);

    const BIT_LIMIT: usize = 4 * 8 + 1;
    let mut bit_count: usize = 0;
    let mut frame_counter: u64 = 0;
    let mut bits = [Bit::Low; BIT_LIMIT];
    'main: loop {
        if bit_count >= BIT_LIMIT {
            bit_count = 0;
            continue 'main;
        }
        let bit = match read_bit(&data_read_pin, &mut delay) {
            Ok(bit) => bit,
            Err(()) => {
                bit_count = 0;
                continue 'main;
            }
        };
        bits[bit_count] = bit;
        bit_count += 1;

        if let Bit::Stop = bit {
            if match_bit_pattern(&bits[..bit_count], &REQUEST_1) {
                send_data(&mut data_write_pin, &mut delay, &RESPONSE_1);
            } else if match_bit_pattern(&bits[..bit_count], &REQUEST_2) {
                send_data(&mut data_write_pin, &mut delay, &RESPONSE_2);
            } else if match_bit_pattern(&bits[..bit_count], &REQUEST_3) {
                frame_counter += 1;
                send_data(
                    &mut data_write_pin,
                    &mut delay,
                    &get_input(frame_counter).into_gc_bits(),
                )
            }
            wait_command(&mut data_read_pin, &mut delay);
            bit_count = 0;
        }
    }
}
