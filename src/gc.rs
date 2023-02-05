// source: testing and https://www.int03.co.uk/crema/hardware/gamecube/gc-control.html
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp_pico::hal::gpio::{FloatingInput, Pin, PinId, PushPullOutput};

#[derive(Clone, Copy, PartialEq)]
pub enum Bit {
    Low,
    High,
    Stop,
    Unknown,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        match value {
            true => Bit::High,
            false => Bit::Low,
        }
    }
}

pub struct ControllerState {
    // 0                // byte 1
    // 0
    // 0/1
    pub start: bool,
    pub y: bool,
    pub x: bool,
    pub b: bool,
    pub a: bool,

    // 1                // byte 2
    pub l: bool,
    pub r: bool,
    pub z: bool,
    pub d_up: bool,
    pub d_down: bool,
    pub d_right: bool,
    pub d_left: bool,

    pub joystick_x: u8, // byte 3
    pub joystick_y: u8, // byte 4
    pub c_stick_x: u8,  // byte 5
    pub c_stick_y: u8,  // byte 6
    pub l_analog: u8,   // byte 7 (possible 4-bit mode)
    pub r_analog: u8,   // byte 8 (possible 4-bit mode)
}

impl ControllerState {
    pub fn into_gc_bits(&self) -> [Bit; 8 * 8 + 1] {
        [
            // Byte 1
            L,
            L,
            L,
            Bit::from(self.start),
            Bit::from(self.y),
            Bit::from(self.x),
            Bit::from(self.b),
            Bit::from(self.a),
            // Byte 2
            H,
            Bit::from(self.l),
            Bit::from(self.r),
            Bit::from(self.z),
            Bit::from(self.d_up),
            Bit::from(self.d_down),
            Bit::from(self.d_right),
            Bit::from(self.d_left),
            // Byte 3
            Bit::from(self.joystick_x & (1 << 7) != 0),
            Bit::from(self.joystick_x & (1 << 6) != 0),
            Bit::from(self.joystick_x & (1 << 5) != 0),
            Bit::from(self.joystick_x & (1 << 4) != 0),
            Bit::from(self.joystick_x & (1 << 3) != 0),
            Bit::from(self.joystick_x & (1 << 2) != 0),
            Bit::from(self.joystick_x & (1 << 1) != 0),
            Bit::from(self.joystick_x & (1 << 0) != 0),
            // Byte 4
            Bit::from(self.joystick_y & (1 << 7) != 0),
            Bit::from(self.joystick_y & (1 << 6) != 0),
            Bit::from(self.joystick_y & (1 << 5) != 0),
            Bit::from(self.joystick_y & (1 << 4) != 0),
            Bit::from(self.joystick_y & (1 << 3) != 0),
            Bit::from(self.joystick_y & (1 << 2) != 0),
            Bit::from(self.joystick_y & (1 << 1) != 0),
            Bit::from(self.joystick_y & (1 << 0) != 0),
            // Byte 5
            Bit::from(self.c_stick_x & (1 << 7) != 0),
            Bit::from(self.c_stick_x & (1 << 6) != 0),
            Bit::from(self.c_stick_x & (1 << 5) != 0),
            Bit::from(self.c_stick_x & (1 << 4) != 0),
            Bit::from(self.c_stick_x & (1 << 3) != 0),
            Bit::from(self.c_stick_x & (1 << 2) != 0),
            Bit::from(self.c_stick_x & (1 << 1) != 0),
            Bit::from(self.c_stick_x & (1 << 0) != 0),
            // Byte 6
            Bit::from(self.c_stick_y & (1 << 7) != 0),
            Bit::from(self.c_stick_y & (1 << 6) != 0),
            Bit::from(self.c_stick_y & (1 << 5) != 0),
            Bit::from(self.c_stick_y & (1 << 4) != 0),
            Bit::from(self.c_stick_y & (1 << 3) != 0),
            Bit::from(self.c_stick_y & (1 << 2) != 0),
            Bit::from(self.c_stick_y & (1 << 1) != 0),
            Bit::from(self.c_stick_y & (1 << 0) != 0),
            // Byte 7
            Bit::from(self.l_analog & (1 << 7) != 0),
            Bit::from(self.l_analog & (1 << 6) != 0),
            Bit::from(self.l_analog & (1 << 5) != 0),
            Bit::from(self.l_analog & (1 << 4) != 0),
            Bit::from(self.l_analog & (1 << 3) != 0),
            Bit::from(self.l_analog & (1 << 2) != 0),
            Bit::from(self.l_analog & (1 << 1) != 0),
            Bit::from(self.l_analog & (1 << 0) != 0),
            // Byte 8
            Bit::from(self.r_analog & (1 << 7) != 0),
            Bit::from(self.r_analog & (1 << 6) != 0),
            Bit::from(self.r_analog & (1 << 5) != 0),
            Bit::from(self.r_analog & (1 << 4) != 0),
            Bit::from(self.r_analog & (1 << 3) != 0),
            Bit::from(self.r_analog & (1 << 2) != 0),
            Bit::from(self.r_analog & (1 << 1) != 0),
            Bit::from(self.r_analog & (1 << 0) != 0),
            S,
        ]
    }
}

impl Default for ControllerState {
    fn default() -> Self {
        Self {
            start: false,
            y: false,
            x: false,
            b: false,
            a: false,
            l: false,
            r: false,
            z: false,
            d_up: false,
            d_down: false,
            d_right: false,
            d_left: false,
            joystick_x: 127,
            joystick_y: 127,
            c_stick_x: 127,
            c_stick_y: 127,
            l_analog: 0,
            r_analog: 0,
        }
    }
}

const L: Bit = Bit::Low;
const H: Bit = Bit::High;
const S: Bit = Bit::Stop;
const U: Bit = Bit::Unknown;

pub fn wait_command<I: PinId>(pin: &Pin<I, FloatingInput>, delay: &mut Delay) {
    while pin.is_high().unwrap() {
        delay.delay_us(1);
    }
}

pub fn read_bit<I: PinId>(pin: &Pin<I, FloatingInput>, delay: &mut Delay) -> Result<Bit, ()> {
    const STOP_BIT_TRESHOLD: u8 = 4;
    const INTERRUPT_TRESHOLD: u8 = 100;
    let mut low_duration: u8 = 1;
    let mut high_duration: u8 = 0;

    while pin.is_low().unwrap() {
        if low_duration > INTERRUPT_TRESHOLD {
            return Err(());
        }
        low_duration += 1;
        delay.delay_us(1);
    }

    while pin.is_high().unwrap() {
        if high_duration > STOP_BIT_TRESHOLD {
            return Ok(Bit::Stop);
        }
        high_duration += 1;
        delay.delay_us(1);
    }

    Ok(match low_duration > high_duration {
        true => Bit::Low,
        false => Bit::High,
    })
}

pub const REQUEST_1: [Bit; 8 + 1] = [
    L, L, L, L, L, L, L, L, // Byte 1
    S,
];

pub const RESPONSE_1: [Bit; 3 * 8 + 1] = [
    L, L, L, L, H, L, L, H, // Byte 1
    L, L, L, L, L, L, L, L, // Byte 2
    L, H, L, L, L, L, L, L, // Byte 3
    S,
];

pub const REQUEST_2: [Bit; 8 + 1] = [
    L, H, L, L, L, L, L, H, // Byte 1
    S,
];

pub const RESPONSE_2: [Bit; 10 * 8 + 1] = [
    L, L, L, L, L, L, L, L, // Byte 1
    H, H, L, L, L, L, L, L, // Byte 2
    L, H, H, H, H, H, H, H, // Byte 3
    L, H, H, H, H, H, H, H, // Byte 4
    L, H, H, H, H, H, H, H, // Byte 5
    L, H, H, H, H, H, H, H, // Byte 6
    L, L, L, L, L, L, L, L, // Byte 7
    L, L, L, L, L, L, L, L, // Byte 8
    L, L, L, L, L, L, L, L, // Byte 9
    L, L, L, L, L, L, L, L, // Byte 10
    S,
];

pub const REQUEST_3: [Bit; 3 * 8 + 1] = [
    L, H, L, L, L, L, L, L, // Byte 1
    L, L, L, L, L, L, H, H, // Byte 2
    L, L, L, L, L, L, U, U, // Byte 3
    S,
];

pub fn get_input(frame: u64) -> ControllerState {
    let mut state = ControllerState::default();
    let sim_frame = (frame / 7) % 2;
    state.joystick_x = if sim_frame == 0 { 0 } else { u8::MAX };
    state
}

pub fn match_bit_pattern(bits: &[Bit], pattern: &[Bit]) -> bool {
    if bits.len() != pattern.len() {
        return false;
    }
    bits.iter()
        .zip(pattern.iter())
        .all(|(a, b)| if *b == U { true } else { *a == *b })
}

pub fn send_data<I: PinId>(pin: &mut Pin<I, PushPullOutput>, delay: &mut Delay, data: &[Bit]) {
    for bit in data {
        let (on_us, off_us) = match bit {
            Bit::Low => (3, 1),
            Bit::High => (1, 3),
            Bit::Stop => (2, 6),
            Bit::Unknown => continue,
        };
        pin.set_high().unwrap();
        delay.delay_us(on_us);
        pin.set_low().unwrap();
        delay.delay_us(off_us);
    }
}
