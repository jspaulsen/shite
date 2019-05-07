use std::process::exit;

use sdl2::EventPump;
use sdl2::event::Event;

use super::context::Context;
use crate::state::GameState;



pub struct GameEvents {
    event_pump: EventPump,
}

// TODO(#17): Track Input (M/KB) state here
// Track Keyboard and mouse (and maybe joystick) state here
// as passing KeyboardState as a parameter within match => won't work.
impl GameEvents {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
        }
    }

    pub fn process_physics_events(&mut self, _game_state: &mut GameState, _context: &mut Context) -> Result<(), String> {
        Ok(())
    }

    pub fn process_sdl_events(&mut self, game_state: &mut GameState, context: &mut Context) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            // TODO: [GameEvents] Implement all events
            match event {
                // TODO: [GameEvents] Route events to higher level
                // These should be routed to a higher level "Application" level
                // provided to the game engine on creation
                Event::Quit {..} => {
                    exit(0);
                },
                Event::KeyUp { .. } => {
                    game_state.on_key_up(context, &event)
                },
                Event::KeyDown { .. } => {
                    game_state.on_key_down(context, &event)
                },
                Event::MouseMotion { .. } => {
                    game_state.on_mouse_motion(context, &event)
                },
                Event::MouseButtonDown { .. } => {
                    game_state.on_mouse_button_down(context, &event)
                },
                Event::MouseButtonUp { .. } => {
                    game_state.on_mouse_button_up(context, &event)
                },
                Event::MouseWheel { .. } => {
                    game_state.on_mouse_wheel(context, &event)
                }
                _ => println!("Uncaptured event {:?}", event)
            }
        }

        Ok(())
    }
}
