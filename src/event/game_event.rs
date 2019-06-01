use sdl2::event::Event;
use sdl2::keyboard::{
    Keycode,
    Mod,
};
use sdl2::mouse::{
    MouseButton,
    MouseState,
    MouseWheelDirection,
};

pub enum GameApplicationEvent {
    Quit { timestamp: u32 },
    Terminating { timestamp: u32 },
    LowMemory { timestamp: u32 },
    WillEnterBackground { timestamp: u32 },
    EnteredBackground { timestamp: u32 },
    WillEnterForeground { timestamp: u32 },
    EnteredForeground { timestamp: u32 },
    Unknown,
}

pub struct GameKeyboardEvent {
    pub timestamp: u32,
    pub keycode: Option<Keycode>,
    pub keymod: Mod,
    pub repeat: bool,
}

pub struct GameTextEditEvent {
    pub timestamp: u32,
    pub text: String,
    pub start: i32,
    pub length: i32,
}

pub struct GameTextInputEvent {
    pub timestamp: u32,
    pub text: String,
}

pub struct GameMouseMotionEvent {
    pub timestamp: u32,
    pub mousestate: MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

pub struct GameMouseButtonEvent {
    pub timestamp: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

pub struct GameMouseWheelEvent {
    pub timestamp: u32,
    pub x: i32,
    pub y: i32,
    pub direction: MouseWheelDirection,
}

pub fn sdl_app_event_to_lib(event: &Event) -> GameApplicationEvent {
    match event {
        Event::Quit {
            timestamp,
        } => GameApplicationEvent::Quit {
            timestamp: *timestamp,
        },
        Event::AppTerminating {
            timestamp,
        } => GameApplicationEvent::Terminating {
            timestamp: *timestamp,
        },
        Event::AppWillEnterBackground {
            timestamp,
        } => GameApplicationEvent::WillEnterBackground {
            timestamp: *timestamp,
        },
        Event::AppDidEnterBackground {
            timestamp,
        } => GameApplicationEvent::EnteredBackground {
            timestamp: *timestamp,
        },
        Event::AppWillEnterForeground {
            timestamp,
        } => GameApplicationEvent::WillEnterForeground {
            timestamp: *timestamp,
        },
        Event::AppDidEnterForeground {
            timestamp,
        } => GameApplicationEvent::EnteredForeground {
            timestamp: *timestamp,
        },
        _ => GameApplicationEvent::Unknown,
    }
}

impl GameKeyboardEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        match event {
            Event::KeyUp {
                timestamp,
                keycode,
                keymod,
                repeat,
                ..
            } => Self {
                timestamp: *timestamp,
                keycode: *keycode,
                keymod: *keymod,
                repeat: *repeat,
            },
            Event::KeyDown {
                timestamp,
                keycode,
                keymod,
                repeat,
                ..
            } => Self {
                timestamp: *timestamp,
                keycode: *keycode,
                keymod: *keymod,
                repeat: *repeat,
            },
            _ => panic!("GameKeyboardEvent::from_sdl_event for inappropriate event"),
        }
    }
}

impl GameTextEditEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        if let Event::TextEditing {
            timestamp,
            text,
            start,
            length,
            ..
        } = event
        {
            Self {
                timestamp: *timestamp,
                text: text.to_string(),
                start: *start,
                length: *length,
            }
        } else {
            panic!("GameTextEditEvent::from_sdl_event called with inappropriate event")
        }
    }
}

impl GameTextInputEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        if let Event::TextInput {
            timestamp,
            text,
            ..
        } = event
        {
            Self {
                timestamp: *timestamp,
                text: text.to_string(),
            }
        } else {
            panic!("GameTextInputEvent::from_sdl_event called with inappropriate event")
        }
    }
}

impl GameMouseMotionEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        if let Event::MouseMotion {
            timestamp,
            mousestate,
            x,
            y,
            xrel,
            yrel,
            ..
        } = event
        {
            Self {
                timestamp: *timestamp,
                mousestate: *mousestate,
                x: *x,
                y: *y,
                xrel: *xrel,
                yrel: *yrel,
            }
        } else {
            panic!("GameMouseMotionEvent::from_sdl_event called with inappropriate event")
        }
    }
}

impl GameMouseButtonEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        match event {
            Event::MouseButtonUp {
                timestamp,
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => Self {
                timestamp: *timestamp,
                mouse_btn: *mouse_btn,
                clicks: *clicks,
                x: *x,
                y: *y,
            },
            Event::MouseButtonDown {
                timestamp,
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => Self {
                timestamp: *timestamp,
                mouse_btn: *mouse_btn,
                clicks: *clicks,
                x: *x,
                y: *y,
            },
            _ => panic!("GameMouseButtonEvent::from_sdl_event called with inappropriate event"),
        }
    }
}

impl GameMouseWheelEvent {
    pub fn from_sdl_event(event: &Event) -> Self {
        if let Event::MouseWheel {
            timestamp,
            x,
            y,
            direction,
            ..
        } = event
        {
            Self {
                timestamp: *timestamp,
                x: *x,
                y: *y,
                direction: *direction,
            }
        } else {
            panic!("GameMouseWheelEvent::from_sdl_event called with inappropriate event")
        }
    }
}
