#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;

///An eight segment display that can display a single digit from 0x0 to 0xF at a time.
/// Intended for use with the HDSP-H101 and HDSP-H103.
///
/// The labels on the segments are as described in the H101 and H103 datasheet:
///```text
///    _______
///   |   a   |
///   |f     b|
///   |   g   |
///   |-------|
///   |e     c|
///   |   d   |
///   '-------'  .
///              p
///```
///Some like the H101 have the segment turned on being pin-low, and some like the H103 have on being pin-high.
///Set the `high_on` boolean as appropriate.
///
/// # Examples
///```rust,ignore
///    // in this case using the `atsamd21_hal` library to provide access to the pins
///    // but any embedded-hal implementation should work
///    let mut peripherals = Peripherals::take().unwrap();
///    let mut pins = peripherals.PORT.split();
///    let mut seg_a = pins.pa21.into_open_drain_output(&mut pins.port);
///    let mut seg_b = pins.pa20.into_open_drain_output(&mut pins.port);
///    let mut seg_c = pins.pa11.into_open_drain_output(&mut pins.port);
///    let mut seg_d = pins.pb10.into_open_drain_output(&mut pins.port);
///    let mut seg_e = pins.pb11.into_open_drain_output(&mut pins.port);
///    let mut seg_f = pins.pa16.into_open_drain_output(&mut pins.port);
///    let mut seg_g = pins.pa17.into_open_drain_output(&mut pins.port);
///    let mut seg_p = pins.pa10.into_open_drain_output(&mut pins.port);
///    let mut eight_segment = EightSegment {
///        high_on: false,
///        seg_a: &mut seg_a,
///        seg_b: &mut seg_b,
///        seg_c: &mut seg_c,
///        seg_d: &mut seg_d,
///        seg_e: &mut seg_e,
///        seg_f: &mut seg_f,
///        seg_g: &mut seg_g,
///        seg_p: &mut seg_p,
///    };
///    eight_segment.blank(); // All segments off
///    eight_segment.display(0xb, false); // Display 'b' with decimal point off
///```

pub struct EightSegment<'a> {
    pub high_on: bool,
    pub seg_a: &'a mut OutputPin,
    pub seg_b: &'a mut OutputPin,
    pub seg_c: &'a mut OutputPin,
    pub seg_d: &'a mut OutputPin,
    pub seg_e: &'a mut OutputPin,
    pub seg_f: &'a mut OutputPin,
    pub seg_g: &'a mut OutputPin,
    pub seg_p: &'a mut OutputPin,
}

impl<'a> EightSegment<'a> {
    pub fn blank(&mut self) {
        if self.high_on {self.seg_a.set_low()} else {self.seg_a.set_high()};
        if self.high_on {self.seg_b.set_low()} else {self.seg_b.set_high()};
        if self.high_on {self.seg_c.set_low()} else {self.seg_c.set_high()};
        if self.high_on {self.seg_d.set_low()} else {self.seg_d.set_high()};
        if self.high_on {self.seg_e.set_low()} else {self.seg_e.set_high()};
        if self.high_on {self.seg_f.set_low()} else {self.seg_f.set_high()};
        if self.high_on {self.seg_g.set_low()} else {self.seg_g.set_high()};
        if self.high_on {self.seg_p.set_low()} else {self.seg_p.set_high()};
    }

    pub fn set_segments(&mut self,
                        seg_a_on: bool,
                        seg_b_on: bool,
                        seg_c_on: bool,
                        seg_d_on: bool,
                        seg_e_on: bool,
                        seg_f_on: bool,
                        seg_g_on: bool,
                        seg_p_on: bool) {

        if seg_a_on ^ !self.high_on {self.seg_a.set_high()} else {self.seg_a.set_low()};
        if seg_b_on ^ !self.high_on {self.seg_b.set_high()} else {self.seg_b.set_low()};
        if seg_c_on ^ !self.high_on {self.seg_c.set_high()} else {self.seg_c.set_low()};
        if seg_d_on ^ !self.high_on {self.seg_d.set_high()} else {self.seg_d.set_low()};
        if seg_e_on ^ !self.high_on {self.seg_e.set_high()} else {self.seg_e.set_low()};
        if seg_f_on ^ !self.high_on {self.seg_f.set_high()} else {self.seg_f.set_low()};
        if seg_g_on ^ !self.high_on {self.seg_g.set_high()} else {self.seg_g.set_low()};
        if seg_p_on ^ !self.high_on {self.seg_p.set_high()} else {self.seg_p.set_low()};
    }

    pub fn display(&mut self, count: u8, seg_p_on: bool) {
        let (seg_a_on, seg_f_on, seg_b_on, seg_g_on, seg_e_on, seg_c_on, seg_d_on) = match count {
            0x0 => (true, true, true, false, true, true, true),
            0x1 => (false, false, true, false, false, true, false),
            0x2 => (true, false, true, true, true, false, true),
            0x3 => (true, false,  true, true, false, true, true),
            0x4 => (false, true, true, true, false, true, false),
            0x5 => (true, true, false, true, false, true, true),
            0x6 => (true, true, false, true, true, true, true),
            0x7 => (true, false, true, false, false, true, false),
            0x8 => (true, true, true, true, true, true, true),
            0x9 => (true, true, true, true, false, true, false),
            0xA => (true, true, true, false, true, true, false),
            0xB => (false, true, false, true, true, true, true),
            0xC => (false, false, false, true, true, false, true),
            0xD => (false, false, true, true, true, true, true),
            0xE => (true, true, false, true, true, false, true),
            0xF => (true, true, false, true, true, false, false),
            _ => (true, true, true, true, true, true, true),
        };

        self.set_segments(
            seg_a_on,
            seg_b_on,
            seg_c_on,
            seg_d_on,
            seg_e_on,
            seg_f_on,
            seg_g_on,
            seg_p_on
        );
    }
}
