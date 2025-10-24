use crate::events::{self, OnEvent, Event, KeyboardState, KeyboardEvent};
use crate::drawable::{Drawable};
use crate::{Context, Component};
use crate::layouts::{Enum, Stack, Size, Offset, Padding};
use crate::emitters;

pub type InputField = emitters::Button<_InputField>;

impl InputField {
    pub fn new(
        default: impl Drawable + 'static,
        focus: impl Drawable + 'static,
        hover: Option<impl Drawable + 'static>,
        error: Option<impl Drawable + 'static>,
        content: impl Drawable + 'static,
        height: f32,
        id: uuid::Uuid,
    ) -> Self {
        emitters::Button::_new(_InputField::new(default, focus, hover, error, content, height, id))
    }
}

#[derive(Debug, Component)]
pub struct _InputField(Stack, Enum, Box<dyn Drawable>, #[skip] pub bool, #[skip] bool, #[skip] pub uuid::Uuid);

impl _InputField {
    pub fn new(
        default: impl Drawable + 'static,
        focus: impl Drawable + 'static,
        hover: Option<impl Drawable + 'static>,
        error: Option<impl Drawable + 'static>,
        content: impl Drawable + 'static,
        height: f32,
        id: uuid::Uuid,
    ) -> Self {
        let height = Size::custom(move |h: Vec<(f32, f32)>| (h[1].0.max(height), h[1].1.max(height)));
        let layout = Stack(Offset::Start, Offset::Start, Size::Fit, height, Padding::default());

        let mut items: Vec<(&str, Box<dyn Drawable>)> = Vec::new();
        items.push(("default", Box::new(default)));
        items.push(("focus", Box::new(focus)));
        if let Some(h) = hover { items.push(("hover", Box::new(h))) }
        if let Some(e) = error { items.push(("error", Box::new(e))) }

        _InputField(layout, Enum::new(items, "default"), Box::new(content), false, false, id)
    }
}

impl OnEvent for _InputField {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<events::Button>() {
            let default = if self.3 {"error"} else {"default"};
            match event {
                events::Button::Hover(true) => self.1.display("hover"),
                events::Button::Hover(false) => self.1.display(default),
                events::Button::Pressed(false) => self.1.display(default),
                events::Button::Pressed(true) => {
                    ctx.hardware.haptic();
                    self.1.display("focus");
                }
            }

            if let events::Button::Pressed(x) = event {
                ctx.trigger_event(events::InputField::Select(self.5, *x));
                self.4 = *x;
            }
        } else if let Some(KeyboardEvent{state: KeyboardState::Pressed, key: _}) = event.downcast_ref() {
            return self.4;
        }
        true
    }
}
