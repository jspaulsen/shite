use ncollide2d::events::{ContactEvent};
use sdl2::EventPump;
use sdl2::event::Event;

use super::context::Context;
//use crate::event::KeyboardEvent;
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
                    game_state.on_collision_start(context, coh1, coh2)?;
                },
                GameCollisionEvent::CollisionEnd(coh1, coh2) => {
                    game_state.on_collision_end(context, coh1, coh2)?;
                },
            }
        }

        Ok(())
    }

    pub fn process_sdl_events(&mut self, game_state: &mut GameState, context: &mut Context) -> Result<(), String> {
        let remap: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in remap {
            // TODO(#22): [GameEvents] Implement all events
            match event {
                Event::Quit {..} => {
                    game_state.on_quit(context, &event);
                },
                Event::KeyUp { .. } => {
                    game_state.on_key_release(context, &event,  &self.event_pump.keyboard_state())?;
                },
                Event::KeyDown { .. } => {
                    game_state.on_key_press(context, &event, &self.event_pump.keyboard_state())?;
                },
                Event::MouseMotion { .. } => {
                    game_state.on_mouse_motion(context, &event)?;
                },
                Event::MouseButtonDown { .. } => {
                    game_state.on_mouse_click(context, &event)?;
                },
                Event::MouseButtonUp { .. } => {
                    game_state.on_mouse_click_release(context, &event)?;
                },
                Event::MouseWheel { .. } => {
                    game_state.on_mouse_wheel(context, &event)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/*
fn on_quit(&mut self, _event: &Event) {
    exit(0);
}

/// Called whenever the operating system is terminating the application
///
fn on_terminating(&mut self, _event: &Event) -> Result<(), String> {
    Ok(())
}
fn on_low_memory(&mut self, _event: &Event) -> Result<(), String> {
    Ok(())
}
*/
