use std::process::exit;

use sdl2::{EventPump, EventSubsystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EventHandler {
    event_pump: EventPump,
    event_subsystem: EventSubsystem,
}


impl EventHandler {
    pub fn new(event_subsystem: EventSubsystem, event_pump: EventPump) -> Self {
        Self {
            event_pump,
            event_subsystem,
        }
    }

    pub fn handle_sdl_events(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            // TODO: Implement all events
            match event {
                // TODO: Route events to higher level
                // These should be routed to a higher level "Application" level
                // provided to the game engine on creation
                Event::Quit {..} => {
                    exit(0);
                },
                Event::KeyDown { keycode, .. } => {
                    let code = keycode.expect("Keycode was unexpectedly None!");

                    // Temporary
                    if code == Keycode::Escape {
                        exit(0);
                    }
                },
                _ => println!("Uncaptured event {:?}", event)
            }
        }

        Ok(())
    }
}
