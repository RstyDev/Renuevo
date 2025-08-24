use crate::{
    entities::{LoginForm, LoginResult},
    frontend::{
        lib::{log, HOST},components::notification::Notification,
        structs::{Auth, Global, NotificationProps, NotificationType},
    },
};
use async_std::task::block_on;
use reqwest::{Client, Response, StatusCode};
use sycamore::prelude::*;
use web_sys::SubmitEvent;
const NAME: &'static str = "Login";

#[component(inline_props)]
pub fn Login(global: Signal<Global>, error_message: Signal<NotificationProps>) -> View {
    let name = create_signal(String::new());
    let last_name = create_signal(String::new());
    let password = create_signal(String::new());
    let error_text = create_selector(move || error_message.get_clone().text);
    view! {
        form(class="form", on:submit = move |ev:SubmitEvent|{
            ev.prevent_default();
            // console_log!("Logging... {} {}", name.get_clone(), last_name.get_clone());
            block_on(async move {
                let res: Response = Client::new().post(format!("{}/login", HOST.as_str())).json(&LoginForm{
                    nombre: name.get_clone(),
                    apellido: last_name.get_clone(),
                    password: password.get_clone(),
                }).send().await.unwrap();
                log(NAME, 25, &res);
                match res.status(){
                    StatusCode::OK=>{
                        let token = res.json::<LoginResult>().await.unwrap();
                        error_message.set(NotificationProps{notification_type: NotificationType::Success, text: String::from("Loggeado correctamente")});
                        global.set_fn(move |_| Global{auth:Auth::Logged(token)});
                        // global.set(Global{auth:Auth::Logged(token),notification:global.get_clone().notification})
                    }
                    _ => {
                        let res = res.json::<String>().await.unwrap();
                        log(NAME, 34, &res);
                        error_message.set(NotificationProps{notification_type: NotificationType::Error, text: res.to_owned()})
                    }
                }
            });
        }){
            input(id = "name_input", r#type = "text", bind:value=name, placeholder = "Nombre"){}
            input(id = "last_name_input", r#type = "text", bind:value=last_name, placeholder = "Apellido"){}
            input(id = "password_input", r#type = "password", bind:value=password, placeholder = "Contraseña"){}
            input(id = "submit_login", r#type = "submit", value ="Ingresar"){"Ingresar"}
            Notification(notification = error_message)
            //p(class="Error"){(error_text.get_clone())}
        }
    }
}
