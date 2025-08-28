use crate::entities::{Persona, dtos::PasswordChange};
use crate::frontend::{structs::Auth, lib::log};
use sycamore::prelude::*;
use web_sys::SubmitEvent;
use regex::Regex;
use std::rc::Rc;
use reqwest::Method;
use async_std::task::block_on;
const NAME: &'static str = "Presbiterado";

#[component(inline_props)]
pub fn ChangePassword(
    auth: Signal<Auth>,
    user: Signal<Option<Persona>>,
    error_message: Signal<String>,
) -> View {
    let name = user.with(|u| {
        u.as_ref()
            .map(|p| format!("{} {}", p.nombre(), p.apellido()))
            .unwrap_or_default()
    });
    let current_pass = create_signal(String::new());
    let first_pass = create_signal(String::new());
    let second_pass = create_signal(String::new());
    let notification_type = create_signal(NotificationType::None);
    let not_type_selector = create_selector(move || notification_type.get());
    let regexes = Rc::new([Regex::new(r#".*[A-Z].*"#).unwrap(),Regex::new(r#".*[0-9].*"#).unwrap(),Regex::new("[!\\\\#$%&'\"()*+,\\-./:;<=>?@\\[\\]^_`{|}~]").unwrap()]);
    let r1 = regexes.clone();
    // let mayus_regex = Regex::new(r#".*[A-Z].*"#).unwrap();
    // let number_regex = Regex::new(r#".*[0-9].*"#).unwrap();
    // let special_regex = Regex::new("[!\\\\#$%&'\"()*+,\\-./:;<=>?@\\[\\]^_`{|}~]").unwrap();
    create_effect(move || {
        update_notification(first_pass,second_pass,error_message,notification_type,regexes.as_ref());
    });
    create_effect(move || {
        update_notification(second_pass,first_pass,error_message,notification_type,r1.as_ref());
    });
    let submit = move |e:SubmitEvent| {
        e.prevent_default();
        if not_type_selector.get() == NotificationType::Success {
            // log(NAME,40,&format!("Submitting.. {}{}",first_pass.get_clone_untracked(),second_pass.get_clone_untracked()))
        }
        block_on(async move {
            let id = user.with(|u|u.as_ref().unwrap().id().unwrap().clone());
            let req = PasswordChange {old_password: current_pass.get_clone_untracked(),new_password: first_pass.get_clone_untracked(), id: id.to_owned()};
            // log(NAME,46,&req);
            crate::frontend::lib::request::<bool>(
                format!("api/v1/users/{}/password",id),
                auth.clone(),
                Method::PUT,
                Some(req),
                false,
            )
                .await
                .unwrap();
        })
    };
    view! {
        form(on:submit= submit){
            div(){
                label(r#for="name_pass_change"){"Nombre:"}
                input(id="name_pass_change",disabled=true, value = name){}
            }
            div(){
                label(r#for="current_pass"){"Ingrese su contraseña actual:"}
                input(id="current_pass",r#type="password", bind:value=current_pass){}
            }
            div(){
                label(r#for="first_pass"){"Ingrese su nueva contraseña:"}
                input(id="first_pass",r#type="password", bind:value=first_pass){}
            }
            div(){
                label(r#for="second_pass"){"Vuelva a ingresar su nueva contraseña:"}
                input(id="second_pass",r#type="password", bind:value=second_pass){}
            }
            (match not_type_selector.get(){
                NotificationType::None=>view!{},
                NotificationType::Success=>view!{
                    p(class= "success"){(error_message.get_clone())}
                },
                NotificationType::Error => view!{
                    p(class= "error"){(error_message.get_clone())}
                }
            })
            input(r#type="submit"){"Guardar"}
            // Notification(notification = error_message)
        }
    }
}
fn update_notification(first: Signal<String>, second: Signal<String>, message: Signal<String>, not_type: Signal<NotificationType>, regexes: &[Regex;3]){
    let pass = first.get_clone();
    if pass.len() == 0 && second.get_clone_untracked().len() == 0 {
        message.set(String::new());
        not_type.set(NotificationType::None);
    } else if pass.len() < 6 {
        message.set(String::from("Las contraseña debe tener al menos 6 caracteres"));
        not_type.set(NotificationType::Error);
    } else if !pass.eq(&second.get_clone_untracked()){
        message.set(String::from("Las contraseñas debe coincidir"));
        not_type.set(NotificationType::Error);
    } else if regexes.iter().any(|r| !r.is_match(pass.as_str())) {
        message.set(String::from("La contraseña debe tener al menos una mayúscula, un número y un caracter especial"));
        not_type.set(NotificationType::Error);
    } else {
        message.set(String::from("Las contraseñas coinciden"));
        not_type.set(NotificationType::Success);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum NotificationType {
    None,
    Error,
    Success
}