/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use rsx_shared::traits::{TEvent, TGenericEvent, TKeyboardEvent, TMouseEvent, TUIEvent};
use rsx_shared::types::{DOMNodeRawId, EventType};

pub use longhands::*;
pub use manager::*;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Event {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent)
}

impl TGenericEvent for Event {}

impl TEvent for Event {
    fn ty(&self) -> EventType {
        match self {
            &Event::KeyEvent(ref event) => event.ty(),
            &Event::MouseEvent(ref event) => event.ty()
        }
    }

    fn target(&self) -> DOMNodeRawId {
        match self {
            &Event::KeyEvent(ref event) => event.target(),
            &Event::MouseEvent(ref event) => event.target()
        }
    }
}

impl TUIEvent for Event {
    fn alt_key(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.alt_key(),
            &Event::MouseEvent(ref event) => event.alt_key()
        }
    }

    fn ctrl_key(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.ctrl_key(),
            &Event::MouseEvent(ref event) => event.ctrl_key()
        }
    }

    fn meta_key(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.meta_key(),
            &Event::MouseEvent(ref event) => event.meta_key()
        }
    }

    fn shift_key(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.shift_key(),
            &Event::MouseEvent(ref event) => event.shift_key()
        }
    }
}

impl TKeyboardEvent for Event {
    type KeyCode = KeyCode;

    fn code(&self) -> Self::KeyCode {
        match self {
            &Event::KeyEvent(ref event) => event.code(),
            &Event::MouseEvent(_) => panic!("Not a key event")
        }
    }

    fn key(&self) -> &'static str {
        match self {
            &Event::KeyEvent(ref event) => event.key(),
            &Event::MouseEvent(_) => panic!("Not a key event")
        }
    }

    fn get_modifier_state(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.get_modifier_state(),
            &Event::MouseEvent(_) => panic!("Not a key event")
        }
    }

    fn repeat(&self) -> bool {
        match self {
            &Event::KeyEvent(ref event) => event.repeat(),
            &Event::MouseEvent(_) => panic!("Not a key event")
        }
    }
}

impl TMouseEvent for Event {
    type MouseButton = MouseButton;

    fn button(&self) -> Self::MouseButton {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.button()
        }
    }

    fn client_x(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.client_x()
        }
    }

    fn client_y(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.client_y()
        }
    }

    fn offset_x(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.offset_x()
        }
    }

    fn offset_y(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.offset_y()
        }
    }

    fn page_x(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.page_x()
        }
    }

    fn page_y(&self) -> u32 {
        match self {
            &Event::KeyEvent(_) => panic!("Not a mouse event"),
            &Event::MouseEvent(ref event) => event.page_y()
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    event_type: EventType,
    target: DOMNodeRawId,
    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool
}

impl BaseEvent {
    pub fn new(event_type: EventType, target: DOMNodeRawId, (alt_key, ctrl_key, meta_key, shift_key): ModifierStateData) -> Self {
        BaseEvent {
            target,
            event_type,
            alt_key,
            ctrl_key,
            meta_key,
            shift_key
        }
    }
}

impl TEvent for BaseEvent {
    fn ty(&self) -> EventType {
        self.event_type
    }

    fn target(&self) -> DOMNodeRawId {
        self.target
    }
}

impl TUIEvent for BaseEvent {
    fn alt_key(&self) -> bool {
        self.alt_key
    }

    fn ctrl_key(&self) -> bool {
        self.ctrl_key
    }

    fn meta_key(&self) -> bool {
        self.meta_key
    }

    fn shift_key(&self) -> bool {
        self.shift_key
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    base: BaseEvent,
    code: KeyCode
}

impl KeyEvent {
    pub fn new(base: BaseEvent, code: KeyCode) -> Self {
        KeyEvent { base, code }
    }

    pub fn into_generic(self) -> Event {
        Event::KeyEvent(self)
    }
}

impl TEvent for KeyEvent {
    fn ty(&self) -> EventType {
        self.base.ty()
    }

    fn target(&self) -> DOMNodeRawId {
        self.base.target()
    }
}

impl TUIEvent for KeyEvent {
    fn alt_key(&self) -> bool {
        self.base.alt_key()
    }

    fn ctrl_key(&self) -> bool {
        self.base.ctrl_key()
    }

    fn meta_key(&self) -> bool {
        self.base.meta_key()
    }

    fn shift_key(&self) -> bool {
        self.base.shift_key()
    }
}

impl TKeyboardEvent for KeyEvent {
    type KeyCode = KeyCode;

    fn code(&self) -> Self::KeyCode {
        self.code
    }

    fn key(&self) -> &'static str {
        unimplemented!()
    }

    fn get_modifier_state(&self) -> bool {
        unimplemented!()
    }

    fn repeat(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    base: BaseEvent,
    client_pos: (u32, u32),
    offset_pos: (u32, u32),
    mouse_pos: (u32, u32),
    mouse_button: MouseButton
}

impl MouseEvent {
    pub fn new(base: BaseEvent, mouse_button: MouseButton, mouse_pos: (u32, u32), client_pos: (u32, u32), offset_pos: (u32, u32)) -> Self {
        MouseEvent {
            base,
            mouse_button,
            mouse_pos,
            client_pos,
            offset_pos
        }
    }

    pub fn into_generic(self) -> Event {
        Event::MouseEvent(self)
    }
}

impl TEvent for MouseEvent {
    fn ty(&self) -> EventType {
        self.base.ty()
    }

    fn target(&self) -> DOMNodeRawId {
        self.base.target()
    }
}

impl TUIEvent for MouseEvent {
    fn alt_key(&self) -> bool {
        self.base.alt_key()
    }

    fn ctrl_key(&self) -> bool {
        self.base.ctrl_key()
    }

    fn meta_key(&self) -> bool {
        self.base.meta_key()
    }

    fn shift_key(&self) -> bool {
        self.base.shift_key()
    }
}

impl TMouseEvent for MouseEvent {
    type MouseButton = MouseButton;

    fn button(&self) -> Self::MouseButton {
        self.mouse_button
    }

    fn client_x(&self) -> u32 {
        self.client_pos.0 as u32
    }

    fn client_y(&self) -> u32 {
        self.client_pos.1 as u32
    }

    fn offset_x(&self) -> u32 {
        self.offset_pos.0 as u32
    }

    fn offset_y(&self) -> u32 {
        self.offset_pos.1 as u32
    }

    fn page_x(&self) -> u32 {
        self.mouse_pos.0 as u32
    }

    fn page_y(&self) -> u32 {
        self.mouse_pos.1 as u32
    }
}
