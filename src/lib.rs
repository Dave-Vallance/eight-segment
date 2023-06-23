#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::v2::OutputPin;

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
/// # RP PICO
///``` rust,ignore
///     // using the `rp_pico` crate that re-exports the rp2040_hal crate
///     use rp_pico 
///     use rp_pico::hal::pac
///     use rp_pico::hal::sio::Sio
///     use eight_segment::EightSegment;
///     
///     let mut pac = pac::Peripherals::take().unwrap();
///     let sio = Sio::new(pac.SIO);
/// 
///     let pins = rp_pico::Pins::new(
///         pac.IO_BANK0,
///         pac.PADS_BANK0,
///         sio.gpio_bank0,
///         &mut pac.RESETS,
///     );
///     // Pins 6-9 are the the bottom four pins on the eight segment display
///     // Pins 10-13 are the top four pins on the eight segment display
///     let mut seg_a = pins.gpio8.into_push_pull_output();
///     let mut seg_b = pins.gpio9.into_push_pull_output();
///     let mut seg_c = pins.gpio12.into_push_pull_output();
///     let mut seg_d = pins.gpio11.into_push_pull_output();
///     let mut seg_e = pins.gpio10.into_push_pull_output();
///     let mut seg_f = pins.gpio7.into_push_pull_output();
///     let mut seg_g = pins.gpio6.into_push_pull_output();
///     let mut seg_p = pins.gpio13.into_push_pull_output();
///
///     let mut eight_segment = EightSegment {
///         high_on: true, 
///         seg_a: &mut seg_a,
///         seg_b: &mut seg_b,
///         seg_c: &mut seg_c,
///         seg_d: &mut seg_d,
///         seg_e: &mut seg_e,
///         seg_f: &mut seg_f,
///         seg_g: &mut seg_g,
///         seg_p: &mut seg_p,
///         };
/// 
///    eight_segment.blank(); // All segments off
///    eight_segment.display(0xb, false); // Display 'b' with decimal point off
///```
/// 
use embedded_hal::digital::v2::PinState;
pub struct EightSegment<'a> {
    pub high_on: bool,
    pub seg_a: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_b: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_c: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_d: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_e: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_f: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_g: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
    pub seg_p: &'a mut dyn OutputPin<Error = core::convert::Infallible>,
}

impl<'a> EightSegment<'a> {
    pub fn blank(&mut self) {
        let _ = self.seg_a.set_state(PinState::from(self.high_on));
        let _ = self.seg_b.set_state(PinState::from(self.high_on));
        let _ = self.seg_c.set_state(PinState::from(self.high_on));
        let _ = self.seg_d.set_state(PinState::from(self.high_on));
        let _ = self.seg_e.set_state(PinState::from(self.high_on));
        let _ = self.seg_f.set_state(PinState::from(self.high_on));
        let _ = self.seg_g.set_state(PinState::from(self.high_on));
        let _ = self.seg_p.set_state(PinState::from(self.high_on));
    }

    pub fn set_segments(
        &mut self,
        seg_a_on: bool,
        seg_b_on: bool,
        seg_c_on: bool,
        seg_d_on: bool,
        seg_e_on: bool,
        seg_f_on: bool,
        seg_g_on: bool,
        seg_p_on: bool,
    ) {
        let _ = self.seg_a.set_state(PinState::from(seg_a_on ^ !self.high_on));
        let _ = self.seg_b.set_state(PinState::from(seg_b_on ^ !self.high_on));
        let _ = self.seg_c.set_state(PinState::from(seg_c_on ^ !self.high_on));
        let _ = self.seg_d.set_state(PinState::from(seg_d_on ^ !self.high_on));
        let _ = self.seg_e.set_state(PinState::from(seg_e_on ^ !self.high_on));
        let _ = self.seg_f.set_state(PinState::from(seg_f_on ^ !self.high_on));
        let _ = self.seg_g.set_state(PinState::from(seg_g_on ^ !self.high_on));
        let _ = self.seg_p.set_state(PinState::from(seg_p_on ^ !self.high_on));
    }

    pub fn display(&mut self, count: u8, seg_p_on: bool) {
        let (
            seg_a_on,
            seg_f_on,
            seg_b_on,
            seg_g_on,
            seg_e_on,
            seg_c_on,
            seg_d_on,
        ) = match count {
            0x0 => (true, true, true, false, true, true, true),
            0x1 => (false, false, true, false, false, true, false),
            0x2 => (true, false, true, true, true, false, true),
            0x3 => (true, false, true, true, false, true, true),
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
            seg_p_on,
        );
    }
}