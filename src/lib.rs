use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct PomodoroTimer {
    time_remaining: u32,
    is_running: bool,
}

#[wasm_bindgen]
impl PomodoroTimer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PomodoroTimer {
        PomodoroTimer {
            time_remaining: 25 * 60,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn reset(&mut self) {
        self.time_remaining = 25 * 60;
        self.is_running = false;
    }

    pub fn decrement(&mut self) {
        if self.is_running && self.time_remaining > 0 {
            self.time_remaining -= 1;
        }
    }

    pub fn get_time(&self) -> u32 {
        self.time_remaining
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn format_time(&self) -> String {
        let minutes = self.time_remaining / 60;
        let seconds = self.time_remaining % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}