use crate::entities::{Estado, EstadoCivil, Persona};
use crate::frontend::{
    components::{
        delete_user_form::refresh_users,
        user_card::{ActionOnUser, Mode, UserCard},
    },
    lib::{log, request},
    structs::Auth,
};
use async_std::task::block_on;
use chrono::{Local, NaiveDate};
use reqwest::Method;
use sycamore::prelude::*;
use web_sys::SubmitEvent;

#[component(inline_props)]
pub fn AddUserForm(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let m1 = miembros.clone();
    let miembros2 = miembros.clone();

    block_on(async move {
        refresh_users(miembros, auth.clone()).await;
    });
    let nombre = create_signal(String::new());
    let apellido = create_signal(String::new());
    let estado_civil = create_signal(String::new());
    let nacimiento = create_signal(String::new());
    let submit_fn = move |ev: SubmitEvent| {
        ev.prevent_default();
        let birth = nacimiento.get_clone().parse::<NaiveDate>().unwrap();
        log("Add User Form", 14, &birth);
        let persona = Persona::new(
            None,
            Some(String::from("123456")),
            nombre.get_clone(),
            apellido.get_clone(),
            birth,
            match estado_civil.get_clone().as_str() {
                "Soltero" => EstadoCivil::Soltero,
                "Casado" => EstadoCivil::Casado,
                "Viudo" => EstadoCivil::Viudo,
                _ => panic!("Not possible"),
            },
            Estado::Nuevo,
            Local::now().naive_local().date(),
        );
        log("Add User Form", 16, &persona);
        block_on(async move {
            request::<bool>("api/v1/users/", auth.clone(), Method::POST, Some(persona))
                .await
                .unwrap();
            nombre.set(String::new());
            apellido.set(String::new());
            estado_civil.set(String::new());
            nacimiento.set(String::new());
            refresh_users(miembros2, auth.clone()).await;
        });
    };
    view! {
        form(id="add_user_form",on:submit=submit_fn) {
            div(){
                label(r#for="Nombre"){"Nombre: "}
                input(name="Nombre", required = true, bind:value=nombre)
            }
            div(){
                label(r#for="Apellido"){"Apellido: "}
                input(name="Apellido", required = true, bind:value=apellido){}
            }
            div(){
                label(r#for="Estado Civil"){"Estado Civil: "}
                select(required=true, value="Soltero", bind:value=estado_civil){
                    option(value="Soltero"){"Soltero"}
                    option(value="Viudo"){"Viudo"}
                    option(value="Casado"){"Casado"}
                }
                // input(placeholder="Estado Civil", required = true, bind:value=estado_civil){}
            }
            div(){
                label(r#for="Nacimiento"){"Nacimiento: "}
                input(name="Nacimiento", r#type="date", required = true, bind:value=nacimiento){}
            }

            input(r#type="submit"){"Agregar"}
        }
        (match miembros.get_clone() {
            Some(miembros) => {
                let iter = miembros.into_iter().map(|m|{
                    let action = create_signal(ActionOnUser::None);
                    view!{li(){UserCard(user=m, mode = Mode::View, action = action)}}}).collect::<Vec<View>>();
                view!{
                    ul(id = "miembros"){
                        (iter)
                    }
                }
            },
            None=>view!{},
        })
    }
}
