use core::convert::TryInto;

use crate::{
    clock::{RtcClock, Time},
    display::Display,
};
use embedded_graphics::prelude::*;
use heapless::String;
use ssd1306::{prelude::WriteOnlyDataCommand, size::DisplaySize};
use ufmt::uwrite;

const EDIT: u8 = 8;
const EDIT_H: u8 = 4;
const EDIT_M: u8 = 2;
const EDIT_S: u8 = 1;

#[derive(Copy, Clone)]
pub struct ClockState {
    edit: u8,
    time: Time,
}

impl ClockState {
    pub fn with_time(time: Time) -> Self {
        Self { edit: 0, time }
    }

    pub fn editing(&self) -> bool {
        self.edit & EDIT != 0
    }
}

#[derive(Copy, Clone)]
pub enum View {
    Clock(ClockState),
}

pub struct Gui<I, S>
where
    S: DisplaySize,
{
    display: Display<I, S>,
    pointer: i8,
    menu: [View; 1],
    rerender: bool,
}
impl<I, S> Gui<I, S>
where
    S: DisplaySize,
    I: WriteOnlyDataCommand,
{
    pub fn new(display: Display<I, S>) -> Self {
        Self {
            display,
            menu: [View::Clock(ClockState::with_time(0.into()))],
            pointer: 0,
            rerender: false,
        }
    }

    pub fn forward(&mut self) {
        match self.current_menu_item() {
            View::Clock(mut state) if state.editing() => {
                if state.edit & EDIT_H != 0 {
                    state.time.hours += 1;
                    if state.time.hours > 24 {
                        state.time.hours = 0;
                    }
                }
                if state.edit & EDIT_M != 0 {
                    state.time.minutes += 1;
                    if state.time.minutes > 59 {
                        state.time.minutes = 0;
                    }
                }
                if state.edit & EDIT_S != 0 {
                    state.time.seconds += 1;
                    if state.time.seconds > 59 {
                        state.time.seconds = 0;
                    }
                }
                core::mem::swap(&mut self.menu[0], &mut View::Clock(state));
            }
            _ => {}
        }

        self.rerender = true;
    }

    pub fn backward(&mut self) {
        match self.current_menu_item() {
            View::Clock(mut state) if state.editing() => {
                if state.edit & EDIT_H != 0 {
                    state.time.hours -= 1;
                    if state.time.hours.checked_sub(1).is_none() {
                        state.time.hours = 23;
                    }
                }
                if state.edit & EDIT_M != 0 {
                    state.time.minutes -= 1;
                    if state.time.minutes.checked_sub(1).is_none() {
                        state.time.minutes = 59;
                    }
                }
                if state.edit & EDIT_S != 0 {
                    state.time.seconds -= 1;
                    if state.time.seconds.checked_sub(1).is_none() {
                        state.time.seconds = 59;
                    }
                }
                core::mem::swap(&mut self.menu[0], &mut View::Clock(state));
            }
            _ => {}
        }

        self.rerender = true;
    }

    fn current_menu_item(&self) -> View {
        self.menu[self.pointer as usize]
    }

    pub fn edit_clock(&mut self, clock: &mut RtcClock) {
        match self.current_menu_item() {
            View::Clock(mut state) if state.editing() => {
                clock.set_time(&state.time);
                state.edit = 0;
                core::mem::swap(&mut self.menu[0], &mut View::Clock(state));
            }
            View::Clock(_) => {
                let mut cs = ClockState::with_time(clock.get_time());
                cs.edit |= EDIT | EDIT_H;
                core::mem::swap(&mut self.menu[0], &mut View::Clock(cs));
            }
        }

        self.rerender = true;
    }

    pub fn select(&mut self) {
        match self.current_menu_item() {
            View::Clock(mut state) if state.editing() => {
                let mut tmp = state.edit & 0x7;
                tmp >>= 1;
                if tmp == 0 {
                    tmp = 4;
                }
                state.edit &= !0x7;
                state.edit |= tmp;
                core::mem::swap(&mut self.menu[0], &mut View::Clock(state));
                self.rerender = true;
            }
            _ => {}
        }
    }

    pub fn print_clock(&mut self, clock: &RtcClock) {
        let justify_x = |text: &str| ((128 - text.len() * 10) / 2).try_into().unwrap_or(0);
        match self.current_menu_item() {
            View::Clock(state) => {
                if self.rerender {
                    self.display.clear();
                    self.rerender = false;
                }

                let mut text: String<16> = String::new();
                match state.editing() {
                    false => {
                        let _ = uwrite!(text, "{}", clock.get_time());
                    }
                    true => {
                        let mut x_offset = 0;
                        //display the edit pointer
                        let _ = uwrite!(text, "{}", state.time);

                        if state.edit & EDIT_H != 0 {
                            //underline hours
                            x_offset = 10;
                        }
                        if state.edit & EDIT_M != 0 {
                            //underline minutes
                            x_offset = 40;
                        }
                        if state.edit & EDIT_S != 0 {
                            //underline seconds
                            x_offset = 70;
                        }
                        self.display.print_pointer(
                            &Point::new(justify_x(&text) + x_offset, 26),
                            &Point::new(justify_x(&text) + x_offset - 8, 31),
                            &Point::new(justify_x(&text) + x_offset + 8, 31),
                        );
                    }
                }
                self.display
                    .print_text(&text, justify_x(&text), (32 - 20) / 2);
                self.rerender = true;
            }
        }
    }
}
