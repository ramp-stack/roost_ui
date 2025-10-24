use crate::events::{OnEvent, Event, MouseEvent, MouseState, self};
use crate::drawable::Drawable;
use crate::{Context, Component};
use crate::layouts::Stack;

#[derive(Debug, Component)]
pub struct Button<D: Drawable + OnEvent> {
    layout: Stack,
    pub inner: D,
}

impl<D: Drawable + OnEvent> Button<D> {
    pub fn _new(inner: D) -> Self {
        Button {layout: Stack::default(), inner}
    }
}

impl<D: Drawable + OnEvent> OnEvent for Button<D> {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            let mut event = match event.state {
                MouseState::Pressed if event.position.is_some() => 
                    Some(events::Button::Pressed(true)),
                MouseState::Moved | MouseState::Scroll(..) => 
                    Some(events::Button::Hover(event.position.is_some())),
                MouseState::Released => {
                    match !mustache::IS_MOBILE && event.position.is_some() {
                        true => Some(events::Button::Hover(true)),
                        false => Some(events::Button::Pressed(false)),
                    }
                },
                _ => None
            };

            if let Some(e) = &mut event {self.inner.on_event(ctx, e);}
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct Selectable<D: Drawable + OnEvent> {
    layout: Stack,
    pub inner: D,
    #[skip] group_id: uuid::Uuid,
    #[skip] id: uuid::Uuid,
}

impl<D: Drawable + OnEvent> Selectable<D> {
    pub fn _new(inner: D, group_id: uuid::Uuid) -> Self {
        Selectable {layout: Stack::default(), inner, group_id, id: uuid::Uuid::new_v4()}
    }
}

impl<D: Drawable + OnEvent> OnEvent for Selectable<D> {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent { state: MouseState::Pressed, position: Some(_) } ) = event.downcast_ref::<MouseEvent>() {
            ctx.trigger_event(events::SelectableEvent(self.id, self.group_id));
        } else if let Some(events::SelectableEvent(id, group_id)) = event.downcast_ref::<events::SelectableEvent>() {
            if *group_id == self.group_id {
                self.inner.on_event(ctx, &mut events::Selectable(*id == self.id));
            }
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct Slider<D: Drawable + OnEvent> {
    layout: Stack,
    pub inner: D,
    #[skip] dragging: bool,
}

impl<D: Drawable + OnEvent> Slider<D> {
    pub fn _new(inner: D) -> Self {
        Slider {layout: Stack::default(), inner, dragging: false}
    }
}

impl<D: Drawable + OnEvent> OnEvent for Slider<D> {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent { state, position, }) = event.downcast_ref::<MouseEvent>() {
            let mut event = match state {
                MouseState::Pressed => position.map(|(x, _)|{
                    self.dragging = true;
                    events::Slider::Start(x)
                }),
                MouseState::Released => {
                    self.dragging = false;
                    None
                }
                MouseState::Scroll(..) | MouseState::Moved if self.dragging => {
                    position.map(|(x, _)| events::Slider::Moved(x))
                }
                _ => None
            };
            
            if let Some(e) = &mut event {self.inner.on_event(ctx, e);}
        }
        true
    }
}