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
//Function for event to decide on weather to pass the event to a child, Event can also be modified for the child
/// Implement the `Event` trait to allow a structure to be used in an event query.
pub trait Event: Debug + Downcast {
    /// Optionally return a clone to continue passing the event to children,
    /// or `None` to stop propagation. Can also modify the event before passing it on.
    fn pass(
        self: Box<Self>,
        _ctx: &mut Context,
        children: Vec<((f32, f32), (f32, f32))>,
    ) -> Vec<Option<Box<dyn Event>>>;
}

impl_downcast!(Event); 

/// Represents the different states of the mouse in a [`MouseEvent`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseState { 
    /// The mouse button was pressed.
    Pressed, 
    /// The mouse was moved.
    Moved, 
    /// The mouse button was released.
    Released,
    /// The mouse was released after a long time.
    ReleasedLong,
    /// The mouse was scrolled.
    /// 
    /// The first value is the horizontal scroll amount (x-axis),
    /// and the second value is the vertical scroll amount (y-axis).
    Scroll(f32, f32), 
}

/// Represents the state of a keyboard key in a [`KeyboardEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardState {
    /// A key was pressed.
    Pressed,
    /// A key was released.
    Released,
}

/// # Mouse Event
///
/// `MouseEvent` is triggered whenever the [`MouseState`] changes.
/// 
/// - `position`: The mouse position at the time of the event.  
///   A component receives `Some(position)` only if the event occurred over it;  
///   otherwise, it will be `None`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseEvent {
    pub position: Option<(f32, f32)>,
    pub state: MouseState,
}

impl Event for MouseEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        let mut passed = false;
        children.into_iter().rev().map(|(offset, size)| { // Reverse to click on the top most element
            let position = self.position.and_then(|position| (!passed).then(|| (
                position.0 > offset.0 &&
                position.0 < offset.0+size.0 &&
                 position.1 > offset.1 &&
                position.1 < offset.1+size.1
                ).then(|| {
                    passed = true;
                    (position.0 - offset.0, position.1 - offset.1)
            })).flatten());

            // let position = self.position.map(|position| {
            //     if !passed { passed = true; }
            //     ((position.0 - offset.0).clamp(0.0, size.0),
            //     (position.1 - offset.1).clamp(0.0, size.1))
            // });

            Some(Box::new(MouseEvent{position, state: self.state}) as Box<dyn Event>)
        }).collect::<Vec<_>>().into_iter().rev().collect()
    }
}

/// # Keyboard Event
///
/// `KeyboardEvent` is triggered whenever the [`KeyboardState`] changes.
/// 
/// - `key`: The [`Key`] that triggered the event.
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
/// # Tick Event
///
/// `TickEvent` is emitted on every tick and can be used to perform continuous or repeated actions.
#[derive(Debug, Clone, Copy)]
pub struct TickEvent;
impl Event for TickEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(Box::new(*self) as Box<dyn Event>)).collect()
    }
}

pub(crate) struct EventHandler {
    touching: bool,
    start_touch: Option<(f32, f32)>,
    mouse: (f32, f32),
    scroll: Option<(f32, f32)>,
    hold: Option<Instant>,
    time: Option<Duration>,
    speed: Option<f32>,
}

impl EventHandler {
    pub fn new() -> Self {EventHandler{
        touching: false,
        start_touch: None,
        mouse: (0.0, 0.0),
        scroll: None,
        hold: None,
        time: None,
        speed: None,
    }}

    pub fn on_input(&mut self, scale: &Scale, input: Input) -> Option<Box<dyn Event>> {
        match input {
            Input::Tick => {
                if !self.touching {
                    if let Some(time) = self.time {
                        match &mut self.speed {
                            Some(speed) => {
                                *speed *= 0.92;
                                if speed.abs() < 0.1 {
                                    self.time = None;
                                    self.speed = None;
                                    self.start_touch = None;
                                    return None;
                                }
                            }
                            None => {
                                let start_y = self.start_touch.unwrap_or((0.0, 0.0)).1;
                                let end_y = self.scroll.unwrap_or((0.0, 0.0)).1;
                                let y_traveled = end_y - start_y;
                                let time_secs = time.as_secs_f32();
                                self.speed = Some(-((y_traveled / time_secs) * 0.05));
                            }
                        }

                        if let Some(speed) = self.speed {
                            let state = (speed.abs() > 0.01).then_some(
                                MouseState::Scroll(0.0, speed)
                            );

                            if let Some(s) = state {
                                return Some(Box::new(MouseEvent { position: Some(self.mouse), state: s }) as Box<dyn Event>);
                            }
                        }
                    }
                }

                None
            }


            Input::Touch(Touch { location, phase, .. }) => {
                let location = (location.x as f32, location.y as f32);
                let position = (scale.logical(location.0), scale.logical(location.1));
                let event = match phase {
                    TouchPhase::Started => {
                        self.time = None;
                        self.speed = None;
                        self.hold = Some(Instant::now());
                        self.scroll = Some(position);
                        self.touching = true;
                        self.start_touch = Some(position);
                        Some(MouseState::Pressed)
                    },
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.touching = false;
                        self.time = self.hold.map(|h| h.elapsed());

                        let hold = self.hold.map(|start| start.elapsed()).unwrap_or_default();
                        match self.start_touch.map(|l| (position.1 - l.1).abs() < 25.0).unwrap_or(false) && hold < Duration::from_millis(600) {
                            true => Some(MouseState::Released),
                            false => Some(MouseState::ReleasedLong),
                        }
                    },
                    TouchPhase::Moved => {
                        self.scroll.and_then(|(prev_x, prev_y)| {
                            self.scroll = Some(position);
                            let dx = position.0 - prev_x;
                            let dy = position.1 - prev_y;
                            let scroll_x = -dx * 1.0;
                            let scroll_y = -dy * 1.0;
                    
                            (scroll_x.abs() > 0.01 || scroll_y.abs() > 0.01).then_some(
                                MouseState::Scroll(scroll_x, scroll_y)
                            )
                        })
                    }
                }.map(|state| Box::new(MouseEvent{position: Some(position), state}) as Box<dyn Event>);
                self.mouse = position;
                event
            },                
            Input::CursorMoved{position, ..} => {
                let position = (scale.logical(position.0 as f32), scale.logical(position.1 as f32));
                (self.mouse != position).then_some({
                    self.mouse = position;
                    Box::new(MouseEvent{position: Some(position), state: MouseState::Moved})
                })
            },
            Input::Mouse{state, ..} => {
                Some(Box::new(MouseEvent{position: Some(self.mouse), state: match state {
                    ElementState::Pressed => MouseState::Pressed,
                    ElementState::Released => MouseState::Released,
                }}))
            },
            Input::MouseWheel{delta, phase, ..} => {
                match phase {
                    TouchPhase::Started => {
                        self.scroll = Some((0.0, 0.0));
                        None
                    }
                    TouchPhase::Moved => {
                        self.scroll.map(|(prev_x, prev_y)| {
                            let pos = match delta {
                                MouseScrollDelta::LineDelta(x, y) => (x.signum(), y.signum()),
                                MouseScrollDelta::PixelDelta(p) => (p.x as f32, p.y as f32),
                            };

                            let scroll_x = prev_x + (-pos.0 * 0.2);
                            let scroll_y = prev_y + (-pos.1 * 0.2);

                            Some(Box::new(MouseEvent{position: Some(self.mouse), state: MouseState::Scroll(scroll_x, scroll_y)}) as Box<dyn Event>)
                        })?
                    },
                    // TouchPhase::Ended => None,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Button {
    Pressed(bool),
    Hover(bool),
}

impl Event for Button {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SelectableEvent(pub uuid::Uuid, pub uuid::Uuid);

impl Event for SelectableEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Selectable(pub bool);

impl Event for Selectable {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slider {
    Start(f32),
    Moved(f32),
}

impl Event for Slider {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

/// Event used to focus active input field on mobile and enable editing of the text input content.
#[derive(Debug, Clone)]
pub enum InputField {
    Select(bool),
    Submit,
}

impl Event for InputField {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}