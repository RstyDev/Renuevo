use crate::entities::{Estado, EstadoCivil, Persona, Sexo};
use crate::frontend::{
    components::{delete_user_form::refresh_users, user_card::Mode, user_cards::UserCards},
    lib::{log, request},
    structs::Auth,
};
use async_std::task::block_on;
use chrono::{Local, NaiveDate};
use reqwest::Method;
use sycamore::prelude::*;
use web_sys::SubmitEvent;

#[component(inline_props)]
pub fn AddUserForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let miembros2 = miembros.clone();
    // block_on(async move {
    //     refresh_users(miembros, auth.clone()).await;
    // });
    let nombre = create_signal(String::new());
    let apellido = create_signal(String::new());
    let sexo = create_signal(String::new());
    create_effect(move || log("Add", 28, &sexo.get_clone()));
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
            match sexo.get_clone().as_str() {
                "Masculino" => Sexo::Masculino,
                "Femenino" => Sexo::Femenino,
                _ => panic!("Not possible"),
            },
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
                label(r#for="Sexo"){"Sexo: "}
                select(name="Sexo", required = true, bind:value=sexo){
                    option(value="Masculino"){"Masculino"}
                    option(value="Femenino"){"Femenino"}
                }
            }
            div(){
                label(r#for="Estado Civil"){"Estado Civil: "}
                select(required=true, value="Soltero", bind:value=estado_civil){
                    option(value="Soltero"){"Soltero/a"}
                    option(value="Viudo"){"Viudo/a"}
                    option(value="Casado"){"Casado/a"}
                }
                // input(placeholder="Estado Civil", required = true, bind:value=estado_civil){}
            }
            div(){
                label(r#for="Nacimiento"){"Nacimiento: "}
                input(name="Nacimiento", r#type="date", required = true, bind:value=nacimiento){}
            }

            input(r#type="submit", value="Agregar"){"Agregar"}
        }
        UserCards(auth=auth.clone(),miembros=miembros.clone(),mode= Mode::View)
    }
}
