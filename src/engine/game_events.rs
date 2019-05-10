use std::process::exit;

use ncollide2d::events::{ContactEvent};
use sdl2::EventPump;
use sdl2::event::Event;

use super::context::Context;
use crate::state::GameState;
use crate::world::GameCollisionEvent;


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

    pub fn process_physics_events(&mut self, game_state: &mut GameState, context: &mut Context) -> Result<(), String> {
        let remap: Vec<GameCollisionEvent> = context.world.contact_events().iter().map(|event| {
            match event {
                ContactEvent::Started(coh1, coh2) => GameCollisionEvent::CollisionStart(*coh1, *coh2),
                ContactEvent::Stopped(coh1, coh2) => GameCollisionEvent::CollisionEnd(*coh1, *coh2),
            }
        }).collect();

        for r in remap {
            match r {
                GameCollisionEvent::CollisionStart(coh1, coh2) => {
                    game_state.on_collision_start(context, coh1, coh2);
                },
                GameCollisionEvent::CollisionEnd(coh1, coh2) => {
                    game_state.on_collision_end(context, coh1, coh2);
                },
            }
        }

        Ok(())
    }

    pub fn process_sdl_events(&mut self, game_state: &mut GameState, context: &mut Context) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            // TODO(#22): [GameEvents] Implement all events
            match event {
                // TODO(#23): [GameEvents] Route events to higher level
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
