use sycamore::{prelude::*, motion::create_tweened_signal, easing};
use crate::frontend::structs::NotificationProps;
use std::time::Duration;

#[component(inline_props)]
pub fn Notification(notification: Signal<NotificationProps>) -> View {
    // let text = create_selector(move || notification.get_clone().text);
    let tweened = create_tweened_signal(500i16,Duration::from_secs(10),easing::sine_in);
    tweened.set(0);
    let message = create_selector(move || notification.get_clone().text);


    view!{
        section(class = notification.get_clone().notification_type.to_string()){
            p(/*style=format!("opacity: {};border: 5px solid;", tweened.get())*/){
                // (tweened.get())
                (message.get_clone())
            }
        }
    }
}

