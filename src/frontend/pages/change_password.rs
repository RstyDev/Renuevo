use sycamore::prelude::*;
use crate::frontend::structs::Auth;
use crate::entities::Persona;
use web_sys::SubmitEvent;

#[component(inline_props)]
pub fn ChangePassword(auth: Signal<Auth>, user: Signal<Option<Persona>>, error_message: Signal<String>) -> View {
    let name = user.with(|u|u.as_ref().map(|p|format!("{} {}",p.nombre(),p.apellido())).unwrap_or_default());
    let current_pass = create_signal(String::new());
    let first_pass = create_signal(String::new());
    let second_pass = create_signal(String::new());
    view!{
        form(on:submit=|e:SubmitEvent|{
            e.prevent_default();
        }){
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
            input(r#type="submit"){"Guardar"}
            // Notification(notification = error_message)
        }
    }
}