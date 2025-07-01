use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use rdev::{listen, simulate, Button, EventType, ListenError};

#[derive(Clone)]
pub struct State {
    active: bool,
    button: Option<Button>,
}

impl State {
    pub fn new() -> Self {
        Self {
            active: true,
            button: None
        }
    }
    
    pub fn stop_it(&mut self) -> bool {
        if self.button.is_some() {
            self.active = false;
            self.button = None;
            return true;
        }
        
        false
    }
    
    pub fn set_button(&mut self, button: Button) {
        if self.active {
            self.button = Some(button);
        }
    }
}

fn main() -> Result<(), ListenError> {
    let state = Arc::new(Mutex::new(State::new()));
    
    {
        let state = state.clone();
        
        spawn(move || {
            loop {
                let state = { state.lock().unwrap().clone() };

                if !state.active {
                    continue
                }
                
                if let Some(button) = state.button {
                    let _ = simulate(&EventType::ButtonPress(button));
                    sleep(Duration::from_millis(5));
                    let _ = simulate(&EventType::ButtonRelease(button));
                    sleep(Duration::from_millis(5));
                }
            }
        });
    }

    {
        listen(move |event| {
            let mut state = state.lock().unwrap();
            
            match event.event_type {
                EventType::ButtonPress(button) => {
                    state.set_button(button);
                }
                EventType::MouseMove { .. } if state.stop_it() => {
                    exit(0)
                }
                _ => {}
            }
        })
    }
}