use crate::events::{self, OnEvent, Event, KeyboardState, KeyboardEvent};
use crate::drawable::{Drawable};
use crate::{Context, Component};
use crate::layouts::{Enum, Stack, Size, Offset, Padding};
use crate::emitters;


#[derive(Debug, Component)]
pub struct InputField(Stack, Enum, Box<dyn Drawable>, #[skip] pub bool, #[skip] bool);

impl InputField {
    pub fn new(
        default: impl Drawable + 'static,
        focus: impl Drawable + 'static,
        hover: Option<impl Drawable + 'static>,
        error: Option<impl Drawable + 'static>,
        content: impl Drawable + 'static,
        height: f32,
    ) -> Self {
        let height = Size::custom(move |h: Vec<(f32, f32)>| (h[1].0.max(height), h[1].1.max(height)));
        let layout = Stack(Offset::Start, Offset::Start, Size::Fit, height, Padding::default());

        let mut items: Vec<(&str, Box<dyn Drawable>)> = Vec::new();
        items.push(("default", Box::new(default)));
        items.push(("focus", Box::new(focus)));
        if let Some(h) = hover { items.push(("hover", Box::new(h))) }
        if let Some(e) = error { items.push(("error", Box::new(e))) }

        InputField(layout, Enum::new(items, "default"), Box::new(content), false, false)
    }
}

impl OnEvent for InputField {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = emitters::Button::get(event) {
            let default = if self.3 {"error"} else {"default"};
            match event {
                events::Button::Hover(true) => self.1.display("hover"),
                events::Button::Pressed(true) => {
                    ctx.hardware.haptic();
                    self.1.display("focus");
                    self.4 = true;
                },
                events::Button::Pressed(false) => {
                    self.4 = false;
                    self.1.display(default);
                },
                _ => self.1.display(default),
            }
        } else if let Some(KeyboardEvent{state: KeyboardState::Pressed, key: _}) = event.downcast_ref() {
            return self.4;
        }
        // } else if let Some(events::SelectableEvent(id, group_id)) = event.downcast_ref::<events::SelectableEvent>() {
        //     if *group_id == uuid::Uuid::new_v3(&uuid::Uuid::NAMESPACE_URL, b"text-input") {
        //         return *id == self.5;
        //     }
        // }
        true
    }
}
