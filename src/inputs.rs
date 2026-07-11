use glfw::WindowEvent;

use crate::math::Vec2;


pub struct Inputs {
    keys: [bool; Keys::LAST_KEY as usize],
    last_keys: [bool; Keys::LAST_KEY as usize],

    mouse_pos: Vec2,

    mouse_scroll_delta: i32,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: [false; Keys::LAST_KEY as usize],
            last_keys: [false; Keys::LAST_KEY as usize],

            mouse_pos: Vec2::ZERO,

            mouse_scroll_delta: 0,
        }
    }

    pub fn new_frame(&mut self) {
        self.mouse_scroll_delta = 0;

        for i in 0..Keys::LAST_KEY as usize {
            self.last_keys[i] = false;
            self.last_keys[i] |= self.keys[i];
        }
    }

    pub fn roll_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                self.keys[*key as usize] = (*action != glfw::Action::Release) && (*key != glfw::Key::Unknown);
            }
            WindowEvent::MouseButton(button, action, _) => {
                self.keys[*button as usize] = *action != glfw::Action::Release
            }
            WindowEvent::CursorPos(x, y) => { self.mouse_pos = Vec2::new(*x as f32, *y as f32) }
            WindowEvent::Scroll(_, y) => { self.mouse_scroll_delta = *y as i32 }
            _ => {}
        }
    }

    pub fn get_mouse_pos(&self) -> Vec2 { self.mouse_pos }


    pub fn get_mouse_scroll(&self) -> i32 { self.mouse_scroll_delta }

    pub fn key_down(&self, key: Keys) -> bool { self.keys[key as usize] }
    pub fn key_pressed(&self, key: Keys) -> bool { self.keys[key as usize] && !self.last_keys[key as usize] }
    pub fn key_release(&self, key: Keys) -> bool { !self.keys[key as usize] && self.last_keys[key as usize] }

    pub fn mouse_down(&self, button: MouseButton) -> bool { self.keys[button as usize] }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool { self.keys[button as usize] && !self.last_keys[button as usize] }
    pub fn mouse_release(&self, button: MouseButton) -> bool { !self.keys[button as usize] && self.last_keys[button as usize] }

    pub fn get_mouse_action(&self, button: MouseButton) -> InputActions {
        if self.mouse_pressed(button) { return InputActions::Pressed }
        if self.mouse_down(button) { return InputActions::Repeat }
        if self.mouse_release(button) { return InputActions::Release }

        return InputActions::Noting;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(unused)]
pub enum Keys {
    Space              = 32,
    Apostrophe         = 39,  // '
    Comma              = 44,  // ,
    Minus              = 45,  // -
    Period             = 46,  // .
    Slash              = 47,  // /
    N0                 = 48,
    N1                 = 49,
    N2                 = 50,
    N3                 = 51,
    N4                 = 52,
    N5                 = 53,
    N6                 = 54,
    N7                 = 55,
    N8                 = 56,
    N9                 = 57,
    Semicolon          = 59,  // ;
    Equal              = 61,  // =
    A                  = 65,
    B                  = 66,
    C                  = 67,
    D                  = 68,
    E                  = 69,
    F                  = 70,
    G                  = 71,
    H                  = 72,
    I                  = 73,
    J                  = 74,
    K                  = 75,
    L                  = 76,
    M                  = 77,
    N                  = 78,
    O                  = 79,
    P                  = 80,
    Q                  = 81,
    R                  = 82,
    S                  = 83,
    T                  = 84,
    U                  = 85,
    V                  = 86,
    W                  = 87,
    X                  = 88,
    Y                  = 89,
    Z                  = 90,
    LeftBracket		   = 91,  // [
    BackSlash		   = 92,  /* \ */
    RightBracket	   = 93,  // ]
    GraveAccent		   = 96,  // `
    World1			   = 161, // non-US #1
    World2			   = 162, // non-US #2

    // Function keys
    Escape             = 256,
    Enter              = 257,
    Tab                = 258,
    BackSpace          = 259,
    Insert             = 260,
    Delete             = 261,
    Right              = 262,
    Left               = 263,
    Down               = 264,
    Up                 = 265,
    PageUp             = 266,
    PageDown           = 267,
    Home               = 268,
    End                = 269,
    CapsLock           = 280,
    ScrollLock         = 281,
    NumLock            = 282,
    Pri32Screen        = 283,
    Pause              = 284,
    F1                 = 290,
    F2                 = 291,
    F3                 = 292,
    F4                 = 293,
    F5                 = 294,
    F6                 = 295,
    F7                 = 296,
    F8                 = 297,
    F9                 = 298,
    F10                = 299,
    F11                = 300,
    F12                = 301,
    F13                = 302,
    F14                = 303,
    F15                = 304,
    F16                = 305,
    F17                = 306,
    F18                = 307,
    F19                = 308,
    F20                = 309,
    F21                = 310,
    F22                = 311,
    F23                = 312,
    F24                = 313,
    F25                = 314,
    Kp0                = 320,
    Kp1                = 321,
    Kp2                = 322,
    Kp3                = 323,
    Kp4                = 324,
    Kp5                = 325,
    Kp6                = 326,
    Kp7                = 327,
    Kp8                = 328,
    Kp9                = 329,
    KpDecimal          = 330,
    KpDivide           = 331,
    KpMultiply         = 332,
    KpSubtract         = 333,
    KpAdd              = 334,
    KpEnter            = 335,
    KpEqual            = 336,
    LeftShift          = 340,
    LeftControl        = 341,
    LeftAlt            = 342,
    LeftSuper          = 343,
    RightShift         = 344,
    RightControl       = 345,
    RightAlt           = 346,
    RightSuper         = 347,
    Menu               = 348,
    LAST_KEY,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InputActions {
    Pressed,
    Repeat,
    Release,
    Noting,
}
