use crate::event::{
    //    GameKeyboardEvent,
    //    GameTextEditEvent,
    //    GameTextInputEvent,
    //    GameMouseMotionEvent,
    GameMouseButtonEvent,
    //    GameMouseWheelEvent,
    UiEvent,
};
use crate::graphics::GameCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;

use super::{
    Button,
    Interfaceable,
    Space,
    Ui,
    UiHandle,
    Widget,
};

pub struct UiWindowBuilder {
    dim_pos: Rect,
    color: Color,
    rel_pos: bool,
    handle: UiHandle,
}

#[derive(Debug)]
pub struct UiWindow {
    bg_color: Color,
    space: Space,
    widgets: HashMap<UiHandle, Box<Widget>>,
    inc: u64,
    focus: Option<UiHandle>,
    handle: UiHandle,
}

impl Ui for UiWindow {
    fn handle(&self) -> UiHandle {
        self.handle
    }

    fn background_color(&self) -> Color {
        self.bg_color
    }

    fn set_background_color(&mut self, color: Color) {
        self.bg_color = color
    }

    fn space(&self) -> &Space {
        &self.space
    }

    fn render(&self, canvas: &mut GameCanvas, rel: Option<Space>) -> Result<(), String> {
        let widgets = self.widgets.iter();
        let space = if let Some(space) = rel { self.space.subtract(&space) } else { self.space.clone() };

        // render ourselves and then render our owned widgets
        canvas.fill_rect(space.rect(), self.bg_color)?;
        widgets.map(|(_, widget)| widget.render(canvas, None)).collect()
    }
}

impl Widget for UiWindow {}

// TODO: [UiWindow] Route uncaptured/unfocused to self

// TOOD: [UiWindow] Implement drag for main element
// This will likely require "draggable" concept
impl Interfaceable for UiWindow {
    fn on_click(&mut self, event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        let widget_opt = self.focus.or_else(|| self.widget_at(event.x, event.y)).and_then(|handle| self.widgets.get_mut(&handle));
        let mut nfocus = None;

        let (capture, ret) = if let Some(widget) = widget_opt {
            nfocus = Some(widget.handle());
            widget.on_click(event)
        } else {
            Ok((false, UiEvent::None))
        }?;

        // take focus
        if capture {
            self.focus = nfocus;
        }

        Ok((true, ret))
    }

    fn on_click_release(&mut self, event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        let widget_opt = self.focus.and_then(|handle| self.widgets.get_mut(&handle));

        let (capture, ret) = if let Some(widget) = widget_opt {
            widget.on_click_release(event)
        } else {
            Ok((false, UiEvent::None))
        }?;

        if !capture {
            self.focus = None;
        }

        Ok((false, ret))
    }
}

// TOOD: [UiWindowBuilder] Is this providing value?
// It does not seem as if UiWindowBuilder is providing any value; consider removing it
impl UiWindowBuilder {
    pub fn new(handle: UiHandle, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            dim_pos: Rect::new(x, y, width, height),
            color: Color::RGB(135, 206, 235),
            rel_pos: false,
            handle,
        }
    }

    /// Use for a base UiWindow which has no parents
    /// Technically, UiWindow isn't expected to have any parents
    /// but that may change in the future.
    pub fn base(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self::new(0, x, y, width, height)
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn relative(mut self, state: bool) -> Self {
        self.rel_pos = state;
        self
    }

    pub fn build(self) -> UiWindow {
        UiWindow {
            bg_color: self.color,
            space: Space::new(self.dim_pos.x(), self.dim_pos.y(), self.dim_pos.width(), self.dim_pos.height(), self.rel_pos),
            widgets: HashMap::new(),
            inc: 0,
            focus: None,
            handle: self.handle,
        }
    }
}

// TODO: [UiWindow] introduce relative position functions for building ui
// Things of this nature would include "middle, top, 50%", etc.

// TODO: [UIWindow] introduce additional widgets and ui elements.
// Some of these widgets may include: Text box, label, checkbox, radio button, menus
impl UiWindow {
    pub fn button(&mut self, x: i32, y: i32, width: u32, height: u32, bg_color: Option<Color>) -> UiHandle {
        let new_handle = self.allocate_handle();

        self.widgets
            .insert(new_handle, Box::new(Button::new(new_handle, Space::new(x, y, width, height, true), bg_color)));

        new_handle
    }

    fn allocate_handle(&mut self) -> UiHandle {
        let ret = self.inc;

        self.inc += 1;
        ret
    }

    fn widget_at(&mut self, x: i32, y: i32) -> Option<UiHandle> {
        for (k, v) in self.widgets.iter() {
            let abs_space = v.space().relative_space(&self.space);

            if abs_space.rect().contains_point((x, y)) {
                return Some(*k);
            }
        }

        None
    }
}
