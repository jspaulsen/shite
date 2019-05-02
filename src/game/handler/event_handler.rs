use std::process::exit;

use sdl2::{EventPump, EventSubsystem};
use sdl2::event::Event;

use crate::game::BoxedGameState;

// TODO(#16): [EventHandler] Don't ignore dead_code
#[allow(dead_code)]
pub struct EventHandler {
    event_pump: EventPump,
    event_subsystem: EventSubsystem,
}

// TODO(#17): Track Input (M/KB) state here
// Track Keyboard and mouse (and maybe joystick) state here
// as passing KeyboardState as a parameter within match => won't work.
impl EventHandler {
    pub fn new(event_subsystem: EventSubsystem, event_pump: EventPump) -> Self {
        Self {
            event_pump,
            event_subsystem,
        }
    }

    pub fn handle_sdl_events(&mut self, game_state: &mut BoxedGameState) -> Result<(), String> {
        for event in &mut self.event_pump.poll_iter() {
            // TODO(#18): [EventHandler] Implement all events
            match event {
                // TODO: Route events to higher level
                // These should be routed to a higher level "Application" level
                // provided to the game engine on creation
                Event::Quit {..} => {
                    exit(0);
                },
                Event::KeyUp { .. } => {
                    game_state.on_key_up(&event)
                },
                Event::KeyDown { .. } => {
                    game_state.on_key_down(&event)
                },
                Event::MouseMotion { .. } => {
                    game_state.on_mouse_motion(&event)
                },
                Event::MouseButtonDown { .. } => {
                    game_state.on_mouse_button_down(&event)
                },
                Event::MouseButtonUp { .. } => {
                    game_state.on_mouse_button_up(&event)
                },
                Event::MouseWheel { .. } => {
                    game_state.on_mouse_wheel(&event)
                }
                _ => println!("Uncaptured event {:?}", event)
            }
        }

        Ok(())
    }
}
