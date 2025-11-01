use crate::events::{Event, MouseEvent, MouseState, KeyboardEvent, KeyboardState};
use crate::{events, Drawable, Context, Component};
use crate::events::OnEvent;
use crate::layouts::Stack;

/// The [`Button`] emitter wraps a drawable component
/// and converts mouse input into a small set of semantic button states:
///
/// - [`Button::Pressed(true)`](crate::events::Button::Pressed) — when the mouse is pressed within the button’s bounds.
/// - [`Button::Pressed(false)`](crate::events::Button::Pressed) — when the mouse is pressed outside the button’s bounds.
/// - [`Button::Hover(true)`](crate::events::Button::Hover) — when the mouse moves over the button.
/// - [`Button::Hover(false)`](crate::events::Button::Hover) — when the mouse leaves the button.
///
/// This allows components to react to common button states without manually handling raw input.
///
#[derive(Debug, Component)]
pub struct Button<D: Drawable + 'static>(Stack, pub D);
impl<D: Drawable + 'static> Button<D> {
    pub fn new(child: D) -> Self {Button(Stack::default(), child)}
}

impl<D: Drawable + 'static> OnEvent for Button<D> {
    fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { 
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            return match event.state {
                MouseState::Pressed if event.position.is_some() => 
                    events![events::Button::Pressed(true)],
                MouseState::Moved | MouseState::Scroll(..) => 
                    events![events::Button::Hover(event.position.is_some())],
                MouseState::Released => {
                    match !roost::IS_MOBILE && event.position.is_some() {
                        true => events![events::Button::Hover(true)],
                        false => events![events::Button::Pressed(false)],
                    }
                },
                _ => Vec::new()
            };
        }
        vec![event]
    }
}

/// The [`Selectable`] emitter allows one item in a group to be active at a time. 
/// When pressed, it emits an event with its unique ID and group ID, 
/// allowing other components in the same group to update their state accordingly.
///
/// - [`Selectable::Pressed(id, group_id)`](crate::events::Selectable::Pressed) - when this element was pressed,
/// - [`Selectable::Selected(true)`](crate::events::Selectable::Selected) - when this element was selected,
/// - [`Selectable::Selected(false)`](crate::events::Selectable::Selected) - when another item in the same group was selected.
#[derive(Debug, Component)]
pub struct Selectable<D: Drawable + 'static>(Stack, pub D, #[skip] uuid::Uuid, #[skip] uuid::Uuid);
impl<D: Drawable + 'static> Selectable<D> {
    pub fn new(child: D, group_id: uuid::Uuid) -> Self {
        Selectable(Stack::default(), child, uuid::Uuid::new_v4(), group_id)
    }
}

impl<D: Drawable + 'static> OnEvent for Selectable<D> {
    fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { 
        if let Some(MouseEvent { state: MouseState::Pressed, position: Some(_) }) = event.downcast_ref::<MouseEvent>() {
            ctx.trigger_event(events::Selectable::Pressed(self.2, self.3));
        } else if let Some(events::Selectable::Pressed(id, group_id)) = event.downcast_ref::<events::Selectable>() {
            if *group_id == self.3 {
                return events![events::Selectable::Selected(*id == self.2)];
            }
        }
        vec![event]
    }
}

/// The [`Slider`] emitter wraps a drawable component
/// and converts mouse input into a small set of semantic slider states:
///
/// - [`Slider::Start(x)`](crate::events::Slider::Start) — when the user clicks or begins dragging.
/// - [`Slider::Moved(x)`](crate::events::Slider::Moved) — while dragging with the mouse pressed.
/// - Automatically stops tracking when released.
#[derive(Debug, Component)]
pub struct Slider<D: Drawable + 'static>(Stack, pub D, #[skip] bool);
impl<D: Drawable + 'static> Slider<D> {
    pub fn new(child: D) -> Self {Slider(Stack::default(), child, false)}
}

impl<D: Drawable + 'static> OnEvent for Slider<D> {
    fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { 
        if let Some(MouseEvent { state, position, }) = event.downcast_ref::<MouseEvent>() {
            return match (state, position) {
                (MouseState::Pressed, Some((x, _))) => {
                    self.2 = true;
                    events![events::Slider::Start(*x)]
                },
                (MouseState::Released, _) => {
                    self.2 = false;
                    Vec::new()
                },
                (MouseState::Scroll(..) | MouseState::Moved, Some((x, _)))
                    if self.2 => {
                    events![events::Slider::Moved(*x)]
                }
                _ => Vec::new()
            };
        }
        vec![event]
    }
}

/// The [`TextInput`] emitter wraps a drawable component
/// and converts raw input into a small set of semantic text input states:
///
/// - [`TextInput::Focused(true)`](crate::events::TextInput::Focused) — when focused (clicked inside bounds).
/// - [`TextInput::Focused(false)`](crate::events::TextInput::Focused) — when unfocused (clicked outside bounds).
/// - [`TextInput::Hover(true)`](crate::events::TextInput::Hover) — when the mouse hovers over the input.
/// - [`TextInput::Hover(false)`](crate::events::TextInput::Hover) — when the mouse leaves the input.
/// - Passes keyboard events through only when focused.
#[derive(Debug, Component)]
pub struct TextInput<D: Drawable + 'static>(Stack, pub D, #[skip] bool);
impl<D: Drawable + 'static> TextInput<D> {
    pub fn new(child: D) -> Self {TextInput(Stack::default(), child, false)}
}

impl<D: Drawable + 'static> OnEvent for TextInput<D> {
    fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        if let Some(e) = event.downcast_ref::<MouseEvent>() {
            let mut events: Vec<Box<dyn Event>> = Vec::new();

            match e.state {
                MouseState::Pressed if e.position.is_some() => {
                    self.2 = true;
                    events.push(Box::new(events::TextInput::Focused(true)));
                }
                MouseState::Pressed if e.position.is_none() => self.2 = false,
                MouseState::Moved | MouseState::Scroll(..) => {
                    events.push(Box::new(events::TextInput::Hover(e.position.is_some())));
                }
                MouseState::Released => {
                    match !roost::IS_MOBILE && e.position.is_some() {
                        true => events.push(Box::new(events::TextInput::Hover(true))),
                        false => events.push(Box::new(events::TextInput::Focused(false))),
                    }
                }
                _ => {}
            }

            events.push(event);

            return events;
        } else if let Some(KeyboardEvent { state: KeyboardState::Pressed, key: _ }) = event.downcast_ref() {
            return if self.2 { vec![event] } else { Vec::new() };
        }

        vec![event]
    }
}
