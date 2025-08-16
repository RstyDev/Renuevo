use crate::entities::{Estado, Familia};
use std::sync::Arc;

use crate::{
    entities::{Persona, Sexo},
    frontend::{
        lib::{log, refresh_users, request},
        structs::Auth,
    },
};
use async_std::task::block_on;
use chrono::{Local, Months};
use reqwest::Method;
use sycamore::prelude::*;

const NAME: &'static str = "Relate Users";

#[component(inline_props)]
pub fn RelateUsersForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let hombres = Arc::from(miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}|Estado::Diacono {..}|Estado::Presbitero {..}=>true,_=>false}&&p.sexo() == Sexo::Masculino&&p.nacimiento()<Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}));
    let opt = hombres.clone();
    let mujeres = Arc::from(miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}=>true,_=>false}&&p.sexo() == Sexo::Femenino&&p.nacimiento()<Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}));
    let padre_actual = create_signal(None);
    let madre_actual = create_signal(None);
    // let familia_actual = create_signal(None);
    let padre_id = create_signal(String::new());
    let madre_id = create_signal(String::new());
    let select_from_father = move |_| {
        block_on(async move {
            let familia = request::<Familia>(format!("api/v1/families/{}",padre_id.get_clone()),auth.clone(),Method::GET,None::<bool>,true).await.unwrap();
            log(NAME,31,&familia);
        })
    };
    let options = match &hombres.as_ref() {
        Some(h) => h
            .into_iter()
            .cloned()
            .map(|p| {
                let id = format!("{}", p.id().unwrap());
                view! {
                    option(value=id){(p.nombre().to_string())}
                }
            })
            .collect::<Vec<View>>(),
        None => return view! {},
    };
    let options_mujeres = match &mujeres.as_ref() {
        Some(h) => h
            .into_iter()
            .cloned()
            .map(|p| {
                let id = format!("{}", p.id().unwrap());
                view! {
                    option(value=id){(p.nombre().to_string())}
                }
            })
            .collect::<Vec<View>>(),
        None => return view! {},
    };
    let hs = hombres.clone();
    create_effect(move ||{
        // padre_id.track();
        if let Some(hombres) = hs.as_ref().clone(){
            let padre_id = padre_id.get_clone();
            padre_actual.set(hombres.into_iter().find(move |p|padre_id.eq(p.id().unwrap())).clone())
        }
    });
    let mjs = mujeres.clone();
    create_effect(move ||{
        if let Some(mujeres) = mjs.as_ref().clone(){
            let madre_id = madre_id.get_clone();
            madre_actual.set(mujeres.into_iter().find(move |m| madre_id.eq(m.id().unwrap())).clone())
        }
    });


    log(NAME, 18, &options);
    create_memo(move || log(NAME, 21, &opt));
    // let options = hombres.map(|p|{p.iter().map(|p|view!{option(value=(p.id())){(p.nombre())}}).collect::<Vec<View>>()});
    view! {
        section(id="padres"){
            article(){
                label(r#for="select_hombre"){"Padre: "}
                select(name="select_hombre", on:change =select_from_father){
                    option(value=""){"-"}
                    (options)
                }
            }
            article(){
                label(r#for="select_mujer"){"Madre: "}
                select(name="select_mujer"){
                    option(value=""){"-"}
                    (options_mujeres)
                }
            }
        }
    }
}

/*
form(){
                        div(){
                            label(r#for="nombre"){"Nombre: "}
                            input(name="nombre",value = user.nombre().to_string(), disabled = true){}
                        }
                        div(){
                            label(r#for="apellido"){"Apellido: "}
                            input(name="apellido", value = u2.apellido().to_string(), disabled = true ){}
                        }
                        div(){
                            label(r#for="nacimiento"){"Nacimiento: "}
                            input(r#type="date", name = "nacimiento", value = u3.nacimiento().to_string(), disabled = true ){}
                        }
                        div(){
                            label(r#for="sexo"){"Sexo: "}
                            select(name="sexo", value = u5.sexo().to_string(), disabled = true) {
                                option(value=""){
                                    (u6.sexo().to_string())
                                }
                            }
                        }
                        div(){
                            label(r#for="estado_civil"){"Estado Civil: "}
                            select(name = "estado_civil", bind:value = estado_civil) {
                                option(value="Soltero"){"Soltero/a"}
                                option(value="Casado"){"Casado/a"}
                                option(value="Viudo"){"Viudo/a"}
                            }
                        }
                        div(){
                            label(r#for="estado"){"Estado: "}
                            select(name = "estado", bind:value = estado) {
                                (match u4.estado(){
                                    Estado::Visitante => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}},
                                    Estado::Nuevo => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                    },
                                    Estado::Fundamentos {..} => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                        option(value = "PreMiembro"){"Pre Miembro"}
                                    },
                                    Estado::PreMiembro {..} => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                        option(value = "PreMiembro"){"Pre Miembro"}
                                        option(value = "Miembro"){"Miembro"}
                                    },
                                    Estado::Miembro {..} => match u4.sexo() {
                                        Sexo::Femenino => view!{
                                            option(value = "Visitante"){"Visitante"}
                                            option(value = "Nuevo"){"Nuevo"}
                                            option(value = "Fundamentos"){"Fundamentos"}
                                            option(value = "PreMiembro"){"Pre Miembro"}
                                            option(value = "Miembro"){"Miembro"}
                                        },
                                        Sexo::Masculino => view!{
                                            option(value = "Visitante"){"Visitante"}
                                            option(value = "Nuevo"){"Nuevo"}
                                            option(value = "Fundamentos"){"Fundamentos"}
                                            option(value = "PreMiembro"){"Pre Miembro"}
                                            option(value = "Miembro"){"Miembro"}
                                            option(value = "Diacono"){"Diácono"}
                                            option(value = "Presbitero"){"Presbítero"}
                                        },
                                    },
                                    Estado::Diacono {..} | Estado::Presbitero {..} => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                        option(value = "PreMiembro"){"Pre Miembro"}
                                        option(value = "Miembro"){"Miembro"}
                                        option(value = "Diacono"){"Diácono"}
                                        option(value = "Presbitero"){"Presbítero"}
                                    }
                                })
                            }
                        }
                        StateForm(estado_numerado = opciones_estado, estado_connector = estado_connector, updated_estado = updated_estado)
                        //aca se sigue
                    }

*/
