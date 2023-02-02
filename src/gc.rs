// source: testing and https://www.int03.co.uk/crema/hardware/gamecube/gc-control.html

pub enum Bit {
    Low,
    High,
    Stop,
}

pub struct BitSamples {
    pub low_samples: u8,
    pub high_samples: u8,
}

pub struct HostCommand {
    // skip 22: 0100 0000 0000 0011 0000 00
    pub rumble_activated_before: bool,
    pub rumble: bool,
    // stop bit
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
