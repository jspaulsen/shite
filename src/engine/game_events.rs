use ncollide2d::events::ContactEvent;
use sdl2::event::Event;
use sdl2::EventPump;

use super::context::Context;
use crate::event::{
    sdl_app_event_to_lib,
    GameKeyboardEvent,
    GameMouseButtonEvent,
    GameMouseMotionEvent,
    GameMouseWheelEvent,
    GameTextEditEvent,
    GameTextInputEvent,
    UiEvent,
};
use crate::state::GameState;
use crate::ui::{
    Interfaceable,
    Ui,
};

use crate::world::GameCollisionEvent;
pub struct GameEvents {
    event_pump: EventPump,
}

impl GameEvents {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
        }
    }

    pub fn process_physics_events(&mut self, game_state: &mut impl GameState, context: &mut Context) -> Result<(), String> {
        let remap: Vec<GameCollisionEvent> = context
            .world
            .contact_events()
            .iter()
            .map(|event| match event {
                ContactEvent::Started(coh1, coh2) => GameCollisionEvent::CollisionStart(*coh1, *coh2),
                ContactEvent::Stopped(coh1, coh2) => GameCollisionEvent::CollisionEnd(*coh1, *coh2),
            })
            .collect();

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

    pub fn process_sdl_events(&mut self, state: &mut impl GameState, ctx: &mut Context) -> Result<(), String> {
        let remap: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in remap {
            match event {
                Event::Quit {
                    ..
                } => {
                    state.on_quit(ctx, &sdl_app_event_to_lib(&event));
                },
                Event::AppTerminating {
                    ..
                } => {
                    state.on_terminating(ctx, &sdl_app_event_to_lib(&event))?;
                },
                Event::AppLowMemory {
                    ..
                } => {
                    state.on_low_memory(ctx, &sdl_app_event_to_lib(&event))?;
                },
                Event::AppWillEnterBackground {
                    ..
                } => {
                    state.on_will_enter_background(ctx, &sdl_app_event_to_lib(&event))?;
                },
                Event::AppDidEnterBackground {
                    ..
                } => {
                    state.on_entered_background(ctx, &sdl_app_event_to_lib(&event))?;
                },
                Event::AppWillEnterForeground {
                    ..
                } => {
                    state.on_will_enter_foreground(ctx, &sdl_app_event_to_lib(&event))?;
                },
                Event::AppDidEnterForeground {
                    ..
                } => {
                    state.on_entered_foreground(ctx, &sdl_app_event_to_lib(&event))?;
                },
                // TODO(#31): Implement GameWindowEvent
                Event::Window {
                    ..
                } => {
                    state.on_window_event(ctx, &event)?;
                },
                Event::KeyDown {
                    ..
                } => {
                    let kbd_event = GameKeyboardEvent::from_sdl_event(&event);
                    let (capture, ui_result) = if let Some(widget) = state.active_ui_mut() {
                        widget.on_key_press(&kbd_event)
                    } else {
                        Ok((false, UiEvent::None))
                    }?;

                    if !capture {
                        state.on_key_press(ctx, &kbd_event, &self.event_pump.keyboard_state())?;
                    }

                    if ui_result != UiEvent::None {
                        state.on_ui_event(ctx, ui_result)?;
                    }
                },
                Event::KeyUp {
                    ..
                } => {
                    let kbd_event = GameKeyboardEvent::from_sdl_event(&event);
                    let (capture, ui_result) = if let Some(widget) = state.active_ui_mut() {
                        widget.on_key_release(&kbd_event)
                    } else {
                        Ok((false, UiEvent::None))
                    }?;

                    if !capture {
                        state.on_key_release(ctx, &kbd_event, &self.event_pump.keyboard_state())?;
                    }

                    if ui_result != UiEvent::None {
                        state.on_ui_event(ctx, ui_result)?;
                    }
                },
                Event::TextEditing {
                    ..
                } => {
                    state.on_text_editing(ctx, &GameTextEditEvent::from_sdl_event(&event))?;
                },
                Event::TextInput {
                    ..
                } => {
                    state.on_text_input(ctx, &GameTextInputEvent::from_sdl_event(&event))?;
                },
                Event::MouseMotion {
                    ..
                } => {
                    let mouse_event = GameMouseMotionEvent::from_sdl_event(&event);
                    state.on_mouse_motion(ctx, &mouse_event)?;
                    //pass_event_to_ui(context, game_state, &event, &self.event_pump)?;
                },
                Event::MouseButtonDown {
                    ..
                } => {
                    let mouse_btn_event = GameMouseButtonEvent::from_sdl_event(&event);

                    let (capture, ui_result) = if let Some(widget) = state.active_ui_mut() {
                        let widget_rect = widget.space().rect();

                        if widget_rect.contains_point((mouse_btn_event.x, mouse_btn_event.y)) {
                            widget.on_click(&mouse_btn_event)
                        } else {
                            Ok((false, UiEvent::None))
                        }
                    } else {
                        Ok((false, UiEvent::None))
                    }?;

                    if !capture {
                        state.on_mouse_click(ctx, &mouse_btn_event)?;
                    }

                    if ui_result != UiEvent::None {
                        state.on_ui_event(ctx, ui_result)?;
                    }
                },
                Event::MouseButtonUp {
                    ..
                } => {
                    let mouse_btn_event = GameMouseButtonEvent::from_sdl_event(&event);
                    let (capture, ui_result) = if let Some(widget) = state.active_ui_mut() {
                        widget.on_click_release(&mouse_btn_event)
                    } else {
                        Ok((false, UiEvent::None))
                    }?;

                    if !capture {
                        state.on_mouse_click_release(ctx, &mouse_btn_event)?;
                    }

                    if ui_result != UiEvent::None {
                        state.on_ui_event(ctx, ui_result)?;
                    }
                },
                Event::MouseWheel {
                    ..
                } => {
                    let mouse_wheel_event = GameMouseWheelEvent::from_sdl_event(&event);
                    state.on_mouse_wheel(ctx, &mouse_wheel_event)?;
                },
                _ => {},
            }
        }

        Ok(())
    }
}

/*
fn pass_event_to_ui(context: &mut Context, state: &mut GameState, evnt: &Event, pump: &EventPump) -> Result<(), String> {
    //let active = state.active_ui();
    let active = context.ui.active();

    for ui in active.iter() {
        match evnt {
            Event::KeyDown { .. } => {
                ui.on_key_press(context, evnt, &pump.keyboard_state())
            }
            Event::KeyUp { .. } => {
                ui.on_key_release(context, evnt, &pump.keyboard_state())
            }
            Event::MouseMotion { .. } => {
                ui.on_mouse_motion(context, evnt)
            }
            Event::MouseButtonDown { x, y, .. } => {
                let rect = ui.rect();

                if rect.contains_point(Point::new(*x, *y)) {
                    ui.on_mouse_click(context, evnt)
                } else {
                    Ok(())
                }
            }
            Event::MouseButtonUp { .. } => {
                ui.on_mouse_click_release(context, evnt)
            }
            Event::MouseWheel { .. } => {
                ui.on_mouse_wheel(context, evnt)
            },
            _ => {Ok(())}
        }?
    }

    Ok(())
}
*/
