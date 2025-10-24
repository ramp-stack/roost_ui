use crate::events::{self, OnEvent, Event};
use crate::drawable::{Drawable};
use crate::{Context, Component};
use crate::layouts::{Enum, Stack};
use crate::emitters;

pub type Selectable = emitters::Selectable<_Selectable>;

impl Selectable {
    pub fn new(
        default: impl Drawable + 'static,
        selected: impl Drawable + 'static,
        is_selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
        group_id: uuid::Uuid,
    ) -> Self {
        emitters::Selectable::_new(_Selectable::new(default, selected, is_selected, on_click), group_id)
    }
}

#[derive(Component)]
pub struct _Selectable(Stack, Enum, #[skip] Box<dyn FnMut(&mut Context)>);

impl _Selectable {
    pub fn new(
        default: impl Drawable + 'static,
        selected: impl Drawable + 'static,
        is_selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let start = if is_selected {"selected"} else {"default"};
        _Selectable(Stack::default(), Enum::new(vec![
            ("default", Box::new(default)),
            ("selected", Box::new(selected)),
        ], start), Box::new(on_click))
    }
}

impl OnEvent for _Selectable {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(events::Selectable(selected)) = event.downcast_ref::<events::Selectable>() {
            match selected {
                false => self.1.display("default"),
                true => {
                    self.1.display("selected");
                    ctx.hardware.haptic();
                    (self.2)(ctx);
                }
            }
        }
        false
    }
}

impl std::fmt::Debug for _Selectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_Selectable")
    }
}