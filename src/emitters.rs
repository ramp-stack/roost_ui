use crate::events::{Event, MouseEvent, MouseState};
use crate::{events, Drawable, Context, Component};
use crate::events::OnEvent;
use crate::layouts::Stack;

#[derive(Debug, Component)]
pub struct Button<D: Drawable + 'static>(Stack, D);
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
                    match !mustache::IS_MOBILE && event.position.is_some() {
                        true => events![events::Button::Hover(true)],
                        false => events![events::Button::Pressed(false)],
                    }
                },
                _ => Vec::new()
            };
        }
        Vec::new()
    }
}

#[derive(Debug, Component)]
pub struct Selectable<D: Drawable + 'static>(Stack, D, #[skip] uuid::Uuid, #[skip] uuid::Uuid);
impl<D: Drawable + 'static> Selectable<D> {
    pub fn new(child: D, group_id: uuid::Uuid) -> Self {
        Selectable(Stack::default(), child, uuid::Uuid::new_v4(), group_id)
    }
}

impl<D: Drawable + 'static> OnEvent for Selectable<D> {
    fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { 
        if let Some(MouseEvent { state: MouseState::Pressed, position: Some(_) }) = event.downcast_ref::<MouseEvent>() {
            ctx.trigger_event(events::SelectableEvent(self.3, self.2));
        } else if let Some(events::SelectableEvent(id, group_id)) = event.downcast_ref::<events::SelectableEvent>() {
            if *group_id == self.3 {
                return events![events::Selectable(*id == self.2)];
            }
        }
        Vec::new()
    }
}

#[derive(Debug, Component)]
pub struct Slider<D: Drawable + 'static>(Stack, D, #[skip] bool);
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
        Vec::new()
    }
}