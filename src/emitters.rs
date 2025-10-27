use crate::events::{Event, MouseEvent, MouseState, self};

pub struct Button;
impl Button {
    pub fn get(event: &mut dyn Event) -> Option<events::Button> {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            return match event.state {
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
        }
        None
    }
}

pub struct Slider;
impl Slider {
    pub fn get(is_dragging: &mut bool, event: &mut dyn Event) -> Option<events::Slider> {
        if let Some(MouseEvent { state, position, }) = event.downcast_ref::<MouseEvent>() {
            return match state {
                MouseState::Pressed => position.map(|(x, _)|{
                    *is_dragging = true;
                    events::Slider::Start(x)
                }),
                MouseState::Released => {
                    *is_dragging = false;
                    None
                },
                MouseState::Scroll(..) | MouseState::Moved if *is_dragging => {
                    position.map(|(x, _)| events::Slider::Moved(x))
                }
                _ => None
            };
        }
        None
    }
}