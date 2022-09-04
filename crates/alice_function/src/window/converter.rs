

// pub fn convert_keyboard_pressed(virtual_keycode: &winit::event::VirtualKeyCode) -> KeyPressedEvent {
//     KeyPressedEvent::new(convert_virtual_key_code(*virtual_keycode), 0)
// }


// pub fn convert_keyboard_released(virtual_keycode: &winit::event::VirtualKeyCode) -> KeyReleasedEvent {
//     KeyReleasedEvent::new(convert_virtual_key_code(*virtual_keycode))

// }

use crate::input::{AliceInputState, MouseButton, KeyCode};

pub fn convert_element_state(element_state: winit::event::ElementState) -> AliceInputState {
    match element_state {
        winit::event::ElementState::Pressed => AliceInputState::Pressed,
        winit::event::ElementState::Released => AliceInputState::Released,
    }
}

pub fn convert_mouse_button(mouse_button: winit::event::MouseButton) -> MouseButton {
 
    match mouse_button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Other(val) => MouseButton::Unknown,
    }

    // MouseButtonPressedEvent::new(button)
}

// pub fn convert_touch_input(
//     touch_input: winit::event::Touch,
//     location: winit::dpi::LogicalPosition<f32>,
// ) -> TouchInput {
//     TouchInput {
//         phase: match touch_input.phase {
//             winit::event::TouchPhase::Started => TouchPhase::Started,
//             winit::event::TouchPhase::Moved => TouchPhase::Moved,
//             winit::event::TouchPhase::Ended => TouchPhase::Ended,
//             winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
//         },
//         position: Vec2::new(location.x as f32, location.y as f32),
//         force: touch_input.force.map(|f| match f {
//             winit::event::Force::Calibrated {
//                 force,
//                 max_possible_force,
//                 altitude_angle,
//             } => ForceTouch::Calibrated {
//                 force,
//                 max_possible_force,
//                 altitude_angle,
//             },
//             winit::event::Force::Normalized(x) => ForceTouch::Normalized(x),
//         }),
//         id: touch_input.id,
//     }
// }

// use crate::function::input::{KeyCode, MouseButton, AliceInputState};

pub fn convert_virtual_key_code(virtual_key_code: winit::event::VirtualKeyCode) -> KeyCode {
    match virtual_key_code {
        winit::event::VirtualKeyCode::Key1 => KeyCode::Key1,
        winit::event::VirtualKeyCode::Key2 => KeyCode::Key2,
        winit::event::VirtualKeyCode::Key3 => KeyCode::Key3,
        winit::event::VirtualKeyCode::Key4 => KeyCode::Key4,
        winit::event::VirtualKeyCode::Key5 => KeyCode::Key5,
        winit::event::VirtualKeyCode::Key6 => KeyCode::Key6,
        winit::event::VirtualKeyCode::Key7 => KeyCode::Key7,
        winit::event::VirtualKeyCode::Key8 => KeyCode::Key8,
        winit::event::VirtualKeyCode::Key9 => KeyCode::Key9,
        winit::event::VirtualKeyCode::Key0 => KeyCode::Key0,
        winit::event::VirtualKeyCode::A => KeyCode::A,
        winit::event::VirtualKeyCode::B => KeyCode::B,
        winit::event::VirtualKeyCode::C => KeyCode::C,
        winit::event::VirtualKeyCode::D => KeyCode::D,
        winit::event::VirtualKeyCode::E => KeyCode::E,
        winit::event::VirtualKeyCode::F => KeyCode::F,
        winit::event::VirtualKeyCode::G => KeyCode::G,
        winit::event::VirtualKeyCode::H => KeyCode::H,
        winit::event::VirtualKeyCode::I => KeyCode::I,
        winit::event::VirtualKeyCode::J => KeyCode::J,
        winit::event::VirtualKeyCode::K => KeyCode::K,
        winit::event::VirtualKeyCode::L => KeyCode::L,
        winit::event::VirtualKeyCode::M => KeyCode::M,
        winit::event::VirtualKeyCode::N => KeyCode::N,
        winit::event::VirtualKeyCode::O => KeyCode::O,
        winit::event::VirtualKeyCode::P => KeyCode::P,
        winit::event::VirtualKeyCode::Q => KeyCode::Q,
        winit::event::VirtualKeyCode::R => KeyCode::R,
        winit::event::VirtualKeyCode::S => KeyCode::S,
        winit::event::VirtualKeyCode::T => KeyCode::T,
        winit::event::VirtualKeyCode::U => KeyCode::U,
        winit::event::VirtualKeyCode::V => KeyCode::V,
        winit::event::VirtualKeyCode::W => KeyCode::W,
        winit::event::VirtualKeyCode::X => KeyCode::X,
        winit::event::VirtualKeyCode::Y => KeyCode::Y,
        winit::event::VirtualKeyCode::Z => KeyCode::Z,
        winit::event::VirtualKeyCode::Escape => KeyCode::Escape,
        winit::event::VirtualKeyCode::F1 => KeyCode::F1,
        winit::event::VirtualKeyCode::F2 => KeyCode::F2,
        winit::event::VirtualKeyCode::F3 => KeyCode::F3,
        winit::event::VirtualKeyCode::F4 => KeyCode::F4,
        winit::event::VirtualKeyCode::F5 => KeyCode::F5,
        winit::event::VirtualKeyCode::F6 => KeyCode::F6,
        winit::event::VirtualKeyCode::F7 => KeyCode::F7,
        winit::event::VirtualKeyCode::F8 => KeyCode::F8,
        winit::event::VirtualKeyCode::F9 => KeyCode::F9,
        winit::event::VirtualKeyCode::F10 => KeyCode::F10,
        winit::event::VirtualKeyCode::F11 => KeyCode::F11,
        winit::event::VirtualKeyCode::F12 => KeyCode::F12,
        winit::event::VirtualKeyCode::F13 => KeyCode::F13,
        winit::event::VirtualKeyCode::F14 => KeyCode::F14,
        winit::event::VirtualKeyCode::F15 => KeyCode::F15,
        winit::event::VirtualKeyCode::F16 => KeyCode::F16,
        winit::event::VirtualKeyCode::F17 => KeyCode::F17,
        winit::event::VirtualKeyCode::F18 => KeyCode::F18,
        winit::event::VirtualKeyCode::F19 => KeyCode::F19,
        winit::event::VirtualKeyCode::F20 => KeyCode::F20,
        winit::event::VirtualKeyCode::F21 => KeyCode::F21,
        winit::event::VirtualKeyCode::F22 => KeyCode::F22,
        winit::event::VirtualKeyCode::F23 => KeyCode::F23,
        winit::event::VirtualKeyCode::F24 => KeyCode::F24,
        winit::event::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
        winit::event::VirtualKeyCode::Scroll => KeyCode::Scroll,
        winit::event::VirtualKeyCode::Pause => KeyCode::Pause,
        winit::event::VirtualKeyCode::Insert => KeyCode::Insert,
        winit::event::VirtualKeyCode::Home => KeyCode::Home,
        winit::event::VirtualKeyCode::Delete => KeyCode::Delete,
        winit::event::VirtualKeyCode::End => KeyCode::End,
        winit::event::VirtualKeyCode::PageDown => KeyCode::PageDown,
        winit::event::VirtualKeyCode::PageUp => KeyCode::PageUp,
        winit::event::VirtualKeyCode::Left => KeyCode::Left,
        winit::event::VirtualKeyCode::Up => KeyCode::Up,
        winit::event::VirtualKeyCode::Right => KeyCode::Right,
        winit::event::VirtualKeyCode::Down => KeyCode::Down,
        winit::event::VirtualKeyCode::Back => KeyCode::Back,
        winit::event::VirtualKeyCode::Return => KeyCode::Return,
        winit::event::VirtualKeyCode::Space => KeyCode::Space,
        winit::event::VirtualKeyCode::Compose => KeyCode::Compose,
        winit::event::VirtualKeyCode::Caret => KeyCode::Caret,
        winit::event::VirtualKeyCode::Numlock => KeyCode::Numlock,
        winit::event::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
        winit::event::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
        winit::event::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
        winit::event::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
        winit::event::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
        winit::event::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
        winit::event::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
        winit::event::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
        winit::event::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
        winit::event::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
        _=> KeyCode::Unknown,
     
    }
}

// pub fn convert_cursor_icon(cursor_icon: CursorIcon) -> winit::window::CursorIcon {
//     match cursor_icon {
//         CursorIcon::Default => winit::window::CursorIcon::Default,
//         CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
//         CursorIcon::Hand => winit::window::CursorIcon::Hand,
//         CursorIcon::Arrow => winit::window::CursorIcon::Arrow,
//         CursorIcon::Move => winit::window::CursorIcon::Move,
//         CursorIcon::Text => winit::window::CursorIcon::Text,
//         CursorIcon::Wait => winit::window::CursorIcon::Wait,
//         CursorIcon::Help => winit::window::CursorIcon::Help,
//         CursorIcon::Progress => winit::window::CursorIcon::Progress,
//         CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
//         CursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
//         CursorIcon::Cell => winit::window::CursorIcon::Cell,
//         CursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
//         CursorIcon::Alias => winit::window::CursorIcon::Alias,
//         CursorIcon::Copy => winit::window::CursorIcon::Copy,
//         CursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
//         CursorIcon::Grab => winit::window::CursorIcon::Grab,
//         CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
//         CursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
//         CursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
//         CursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
//         CursorIcon::EResize => winit::window::CursorIcon::EResize,
//         CursorIcon::NResize => winit::window::CursorIcon::NResize,
//         CursorIcon::NeResize => winit::window::CursorIcon::NeResize,
//         CursorIcon::NwResize => winit::window::CursorIcon::NwResize,
//         CursorIcon::SResize => winit::window::CursorIcon::SResize,
//         CursorIcon::SeResize => winit::window::CursorIcon::SeResize,
//         CursorIcon::SwResize => winit::window::CursorIcon::SwResize,
//         CursorIcon::WResize => winit::window::CursorIcon::WResize,
//         CursorIcon::EwResize => winit::window::CursorIcon::EwResize,
//         CursorIcon::NsResize => winit::window::CursorIcon::NsResize,
//         CursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
//         CursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
//         CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
//         CursorIcon::RowResize => winit::window::CursorIcon::RowResize,
//     }
// }


