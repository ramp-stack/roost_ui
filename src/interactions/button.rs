use crate::events::{self, OnEvent, Event};
use crate::drawable::{Drawable};
use crate::{Context, Component};
use crate::layouts::{Enum, Stack};
use crate::emitters;

#[derive(Component)]
pub struct Button(Stack, Enum, #[skip] pub bool, #[skip] bool, #[skip] Box<dyn FnMut(&mut Context)>);

impl Button {
    pub fn new(
        default: impl Drawable + 'static,
        hover: Option<impl Drawable + 'static>,
        pressed: Option<impl Drawable + 'static>,
        disabled: Option<impl Drawable + 'static>,
        is_disabled: bool,
        on_press: bool, // run callback on click or on releasee
        callback: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let start = if is_disabled {"disabled"} else {"default"};
        let mut items: Vec<(&str, Box<dyn Drawable>)> = Vec::new();
        items.push(("default", Box::new(default)));
        if let Some(h) = hover { items.push(("hover", Box::new(h))) }
        if let Some(p) = pressed { items.push(("pressed", Box::new(p))) }
        if let Some(d) = disabled { items.push(("disabled", Box::new(d))) }
        Button(Stack::default(), Enum::new(items, start), is_disabled, on_press, Box::new(callback))
    }
}

impl OnEvent for Button {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = emitters::Button::get(event) {
            match event {
                events::Button::Hover(true) => self.1.display("hover"),
                events::Button::Hover(false) => self.1.display("default"),
                events::Button::Pressed(false) => self.1.display("default"),
                events::Button::Pressed(true) => self.1.display("pressed"),
            }

            if event == events::Button::Pressed(self.3) {
                ctx.hardware.haptic();
                (self.4)(ctx);
            }

            if self.2 {self.1.display("disabled")}
        }
        false
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button")
    }
}

