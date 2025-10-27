use crate::events::{self, OnEvent, Event};
use crate::drawable::{Drawable};
use crate::{Context, Component};
use crate::layouts::{Enum, Stack};
use crate::emitters;

#[derive(Component)]
pub struct Selectable(Stack, Enum, #[skip] Box<dyn FnMut(&mut Context)>, #[skip] uuid::Uuid, #[skip] uuid::Uuid);

impl Selectable {
    pub fn new(
        default: impl Drawable + 'static,
        selected: impl Drawable + 'static,
        is_selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
        group_id: uuid::Uuid,
    ) -> Self {
        let start = if is_selected {"selected"} else {"default"};
        Selectable(Stack::default(), Enum::new(vec![
            ("default", Box::new(default)),
            ("selected", Box::new(selected)),
        ], start), Box::new(on_click), group_id, uuid::Uuid::new_v4())
    }
}

impl OnEvent for Selectable {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(events::Button::Pressed(true)) = emitters::Button::get(event) {
            ctx.trigger_event(events::SelectableEvent(self.4, self.3));
        } else if let Some(events::SelectableEvent(id, group_id)) = event.downcast_ref::<events::SelectableEvent>() {
            if *group_id == self.3 {
                match *id == self.4 {
                    false => self.1.display("default"),
                    true => {
                        self.1.display("selected");
                        ctx.hardware.haptic();
                        (self.2)(ctx);
                    }
                }
            }
        }
        false
    }
}

impl std::fmt::Debug for Selectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Selectable")
    }
}
