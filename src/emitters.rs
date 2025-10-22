use crate::events::{OnEvent, Event, MouseEvent, MouseState, self};
use crate::drawable::Drawable;
use crate::{Context, Component};
use crate::layouts::Stack;

#[derive(Debug, Component)]
pub struct Button<D: Drawable + OnEvent> {
    l: Stack,
    pub inner: D,
}
impl<D: Drawable + OnEvent> Button<D> {
    pub fn new(inner: D) -> Self {
        Button {l: Stack::default(), inner}
    }
}

impl<D: Drawable + OnEvent> OnEvent for Button<D> {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            let mut event = match (event.state, event.position.is_some()) {
                (MouseState::Pressed, true) => Some(events::Button::Pressed(true)),
                (MouseState::Pressed, false) => None,
                (MouseState::Moved | MouseState::Scroll(..), true) => 
                    Some(events::Button::Hover(true)),
                (MouseState::Moved| MouseState::Scroll(..), false) => 
                    Some(events::Button::Hover(false)),
                (MouseState::Released | MouseState::ReleasedLong, true) => Some( match mustache::IS_MOBILE {
                    true => events::Button::Pressed(false),
                    false => events::Button::Hover(true),
                }),
                (MouseState::Released | MouseState::ReleasedLong, false) => Some(events::Button::Pressed(false)),
            };

            if let Some(e) = &mut event {self.inner.on_event(ctx, e);}
        }
        true
    }
}


// pub type TextInput = emitter::TextInput<InputContent>;

// pub struct InputContent(Stack, Enum, TextEditior);
// impl OnEvent for Button {
//     fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
//         if let Some(event::TextInput(keyboard_event)) = event.downcast::<event::TextInput>() {

//         if let Some(event::Selected(selected)) = event.downcast::<event::Selected>() {
//             if selected {
//                 //choose focused background from enum
//             } else {

//             }
//         }
//             match event {
//                 ButtonEvent::Pressed => self.1.display("pressed"),
//             }
//         }
//     }
// }