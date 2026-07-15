use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use mouce::common::{MouseButton, MouseEvent};
use mouce::{Mouse, MouseActions};

#[derive(Clone)]
pub struct State {
    active: bool,
    button: Option<MouseButton>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            active: true,
            button: None,
        }
    }

    fn stop_it(&mut self) -> bool {
        if self.button.is_some() {
            self.active = false;
            self.button = None;
            return true;
        }
        false
    }

    fn set_button(&mut self, button: MouseButton) {
        if self.active {
            self.button = Some(button);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(State::new()));

    {
        let state = state.clone();
        spawn(move || {
            let mouse = Mouse::new();
            loop {
                let state = { state.lock().unwrap().clone() };

                if !state.active {
                    continue;
                }

                if let Some(button) = state.button {
                    let _ = mouse.press_button(button);
                    sleep(Duration::from_millis(5));
                    let _ = mouse.release_button(button);
                    sleep(Duration::from_millis(5));
                }
            }
        });
    }

    let mut mouse = Mouse::new();
    mouse.hook(Box::new(move |event| {
        let mut state = state.lock().unwrap();
        match event {
            MouseEvent::Press(button) => {
                state.set_button(*button);
            }
            MouseEvent::AbsoluteMove(_, _) if state.stop_it() => {
                exit(0);
            }
            _ => {}
        }
    }))?;

    loop {
        sleep(Duration::from_secs(1));
    }
}
