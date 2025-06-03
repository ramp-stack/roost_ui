use crate::layout::Scale;
use crate::Context;

use std::time::Duration;
use std::time::Instant;
use std::fmt::Debug;

use maverick_os::window::{Input, TouchPhase, ElementState, MouseScrollDelta, Touch};
pub use maverick_os::window::{NamedKey, Key, SmolStr};

use downcast_rs::{Downcast, impl_downcast};

pub type Events = std::collections::VecDeque<Box<dyn Event>>;

pub trait OnEvent: Debug {
    fn on_event(&mut self, _ctx: &mut Context, _event: &mut dyn Event) -> bool {true}
}

pub trait Event: Debug + Downcast {
    ///Function for event to decide on weather to pass the event to a child, Event can also be modified for the child
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>>;
}
impl_downcast!(Event); 

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseState{ Pressed, Moved, Released, LongPressReleased, Scroll(f32, f32) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardState{ Pressed, Released }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseEvent {
    pub position: Option<(f32, f32)>,
    pub state: MouseState,
}

impl Event for MouseEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        let mut passed = false;
        children.into_iter().rev().map(|(offset, size)| {//Reverse to click on the top most element
            let position = self.position.and_then(|position| (!passed).then(|| (
                position.0 > offset.0 &&
                position.0 < offset.0+size.0 &&
                 position.1 > offset.1 &&
                position.1 < offset.1+size.1
                ).then(|| {
                    passed = true;
                    (position.0 - offset.0, position.1 - offset.1)
            })).flatten());
            Some(Box::new(MouseEvent{position, state: self.state}) as Box<dyn Event>)
        }).collect::<Vec<_>>().into_iter().rev().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: KeyboardState,
}

impl Event for KeyboardEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TickEvent;
impl Event for TickEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

pub(crate) struct EventHandler {
    touching: bool,
    touch_start_time: Option<Instant>,
    mouse: (f32, f32),
    scroll: (f32, f32)

}
impl EventHandler {
    pub fn new() -> Self {EventHandler{
        touching: false,
        touch_start_time: None,
        mouse: (0.0, 0.0),
        scroll: (0.0, 0.0),
    }}

    pub fn on_input(&mut self, scale: &Scale, input: Input) -> Option<Box<dyn Event>> {
        match input {
            Input::Touch(Touch { location, phase, .. }) => {
                let location = (location.x as f32, location.y as f32);
                let position = (scale.logical(location.0), scale.logical(location.1));
                let event = match phase {
                    TouchPhase::Started => {
                        self.touching = true;
                        self.touch_start_time = Some(Instant::now());
                        Some(MouseState::Pressed)
                    },
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.touching = false;
                        self.scroll = (0.0, 0.0);

                        let held_for = self.touch_start_time
                            .take()
                            .map(|start| start.elapsed())
                            .unwrap_or_default();

                        if held_for < Duration::from_millis(200) {
                            Some(MouseState::Released)
                        } else {
                            Some(MouseState::LongPressReleased)
                        }
                    },
                    TouchPhase::Moved => {
                        self.touching.then(|| {
                            let dx = location.0 - self.mouse.0;
                            let dy = location.1 - self.mouse.1;
                    
                            let scroll_speed = 0.3; // Tune this to adjust sensitivity
                            let scroll_x = -dx * scroll_speed;
                            let scroll_y = -dy * scroll_speed;
                    
                            (scroll_x.abs() > 0.01 || scroll_y.abs() > 0.01).then_some(
                                MouseState::Scroll(scroll_x, scroll_y)
                            )
                        }).flatten()
                    }
                }.map(|state| Box::new(MouseEvent{position: Some(position), state}) as Box<dyn Event>);
                self.mouse = position;
                event
            },                
            Input::CursorMoved{position, ..} => {
                let position = (scale.logical(position.0 as f32), scale.logical(position.1 as f32));
                (self.mouse != position).then_some(
                    Box::new(MouseEvent{position: Some(position), state: MouseState::Moved})
                )
            },
            Input::Mouse{state, ..} => {
                Some(Box::new(MouseEvent{position: Some(self.mouse), state: match state {
                    ElementState::Pressed => MouseState::Pressed,
                    ElementState::Released => MouseState::Released,
                }}))
            },
            Input::MouseWheel{delta, phase, ..} => {
                match phase {
                    TouchPhase::Moved => {
                        let pos = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(p) => (p.x as f32, p.y as f32),
                        };
                        let scroll_speed = 0.2; // Tune this to adjust sensitivity
                        self.scroll.0 += -pos.0 * scroll_speed;
                        self.scroll.1 += -pos.1 * scroll_speed;
                        Some(Box::new(MouseEvent{position: Some(self.mouse), state: MouseState::Scroll(self.scroll.0, self.scroll.1)}))
                    },
                    TouchPhase::Ended => {
                        self.scroll = (0.0, 0.0);
                        None
                    },
                    _ => None
                }
            },
            Input::Keyboard{event, ..} => {
                Some(Box::new(KeyboardEvent{
                    key: event.logical_key, state: match event.state {
                    ElementState::Pressed => KeyboardState::Pressed,
                    ElementState::Released => KeyboardState::Released,
                }}))
            },
            _ => None
        }
    }
}
