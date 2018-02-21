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

use rsx_shared::traits::{NumCast, TCallback, TClientRect, TDOMNode, TDOMTree, TEventManager, ToPrimitive};
use rsx_shared::types::{Closure, EventType, VirtualEventType};

use types::{BaseEvent, Event, KeyCode, KeyEvent, MouseButton, MouseEvent};

pub type EventListener = Closure<Event>;
pub type ModifierStateData = (bool, bool, bool, bool);
pub type KeyEventData = (ModifierStateData, KeyCode);
pub type MouseEventData = (ModifierStateData, MouseButton, (u32, u32));

#[derive(Debug, PartialEq)]
pub struct Binding<T: TDOMNode> {
    target_id: T::Id,
    event_type: EventType,
    closure: EventListener
}

#[derive(Debug, PartialEq)]
pub struct EventManager<T: TDOMNode> {
    pending_key_events: Vec<(VirtualEventType, KeyEventData)>,
    pending_mouse_events: Vec<(VirtualEventType, MouseEventData)>,
    bindings: Vec<Binding<T>>
}

impl<T> Default for EventManager<T>
where
    T: TDOMNode<Event = Event>,
    T::Id: ToPrimitive + Copy
{
    fn default() -> Self {
        EventManager {
            pending_key_events: Vec::new(),
            pending_mouse_events: Vec::new(),
            bindings: Vec::new()
        }
    }
}

impl<T> EventManager<T>
where
    T: TDOMNode<Event = Event>,
    T::Id: ToPrimitive + Copy
{
    fn synthesize_key_event<A, F>(_: &A, binding: &mut Binding<T>, (modifiers, code): KeyEventData, mut intercept: F)
    where
        A: TDOMTree<Node = T>,
        F: FnMut(Event)
    {
        let target = NumCast::from(binding.target_id).unwrap_or_default();
        let base = BaseEvent::new(binding.event_type, target, modifiers);
        let event = KeyEvent::new(base, code).into_generic();
        binding.closure.call(event).unwrap_or_else(&mut intercept);
    }

    fn synthesize_mouse_event<A, F>(arena: &A, binding: &mut Binding<T>, (modifiers, button, mouse_pos): MouseEventData, mut intercept: F)
    where
        A: TDOMTree<Node = T>,
        F: FnMut(Event)
    {
        let bounding_client_rect = arena
            .get_node(binding.target_id)
            .get_global_bounding_client_rect();

        if !bounding_client_rect.contains_point(mouse_pos) {
            return;
        }

        let target = NumCast::from(binding.target_id).unwrap_or_default();
        let client_pos = bounding_client_rect.client_from_page(mouse_pos);
        let offset_pos = bounding_client_rect.offset_from_page(mouse_pos);
        let base = BaseEvent::new(binding.event_type, target, modifiers);
        let event = MouseEvent::new(base, button, mouse_pos, client_pos, offset_pos).into_generic();
        binding.closure.call(event).unwrap_or_else(&mut intercept);

        // TODO: Also emit synthetic MouseEnter, MouseLeave, MouseOver, MouseOut when necessary
    }
}

impl<T> TEventManager for EventManager<T>
where
    T: TDOMNode<Event = Event>,
    T::Id: ToPrimitive + Copy
{
    type Target = T;
    type KeyCode = KeyCode;
    type MouseButton = MouseButton;
    type KeyEventData = KeyEventData;
    type MouseEventData = MouseEventData;

    fn add_event_listener<F>(&mut self, target_id: <Self::Target as TDOMNode>::Id, event_type: EventType, closure: F)
    where
        F: Into<EventListener>
    {
        self.bindings.push(Binding {
            target_id,
            event_type,
            closure: closure.into()
        });
    }

    fn remove_event_listener<F>(&mut self, target_id: <Self::Target as TDOMNode>::Id, event_type: EventType, closure: F)
    where
        F: Into<EventListener>
    {
        let closure = closure.into();
        let is_different_binding = |v: &Binding<T>| v.target_id != target_id || v.event_type != event_type || v.closure != closure;
        self.bindings.retain(is_different_binding);
    }

    fn receive_key_event(&mut self, event_type: VirtualEventType, event_data: KeyEventData) {
        self.pending_key_events.push((event_type, event_data))
    }

    fn receive_mouse_event(&mut self, event_type: VirtualEventType, event_data: MouseEventData) {
        self.pending_mouse_events.push((event_type, event_data))
    }

    fn broadcast_events<A>(&mut self, arena: &A)
    where
        A: TDOMTree<Node = Self::Target>
    {
        self.intercept_events(arena, |event| println!("Unhandled event: {:?}", event));
        unimplemented!()
    }

    fn intercept_events<A, F>(&mut self, arena: &A, mut intercept: F)
    where
        A: TDOMTree<Node = Self::Target>,
        F: FnMut(Event)
    {
        let pending_key_events = &mut self.pending_key_events;
        let pending_mouse_events = &mut self.pending_mouse_events;
        let bindings = &mut self.bindings;

        pending_key_events
            .drain(..)
            .for_each(|(virtual_event_type, virtual_event_data)| {
                bindings
                    .iter_mut()
                    .filter(|binding| binding.event_type == virtual_event_type)
                    .for_each(|binding| EventManager::synthesize_key_event(arena, binding, virtual_event_data, &mut intercept));
            });

        pending_mouse_events
            .drain(..)
            .for_each(|(virtual_event_type, virtual_event_data)| {
                bindings
                    .iter_mut()
                    .filter(|binding| binding.event_type == virtual_event_type)
                    .for_each(|binding| EventManager::synthesize_mouse_event(arena, binding, virtual_event_data, &mut intercept));
            });
    }
}
