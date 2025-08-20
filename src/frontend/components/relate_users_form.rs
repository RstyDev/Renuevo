use crate::entities::{Estado, Familia};

use crate::{
    entities::{Persona, Sexo},
    frontend::{
        lib::{log, request},
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
    let hombres = create_selector(move || {
        miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}|Estado::Diacono {..}|Estado::Presbitero {..}=>true,_=>false}&&p.sexo() == Sexo::Masculino&&p.nacimiento()<=Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}).unwrap_or_default()
    });
    let mujeres = create_selector(move || {
        miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}=>true,_=>false}&&p.sexo() == Sexo::Femenino&&p.nacimiento()<=Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}).unwrap_or_default()
    });
    let padre_id = create_signal(String::new());
    let madre_id = create_signal(String::new());
    let nuevo_hijo_id = create_signal(String::new());
    let familia_actual: Signal<Option<Familia>> = create_signal(None);
    let hijos_disponibles: ReadSignal<Vec<Persona>> = create_selector(move || {
        let miembros = miembros.get_clone_untracked();
        let familia_actual = familia_actual.get_clone();
        let padre = familia_actual.as_ref().map(|f| f.padre().clone()).flatten();
        // let padre = padre_id.get_clone();
        let madre = familia_actual.as_ref().map(|f| f.madre().clone()).flatten();
        log(NAME, 34, &familia_actual);
        let hijos_actual = familia_actual
            .map(|f| f.hijos().clone())
            .unwrap_or_default();
        miembros
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|m| {
                (padre.is_some() || madre.is_some())
                    && padre
                        .as_ref()
                        .map(|p| !p.id().unwrap().eq(m.id().unwrap()) && m.is_possible_son_of(p))
                        .unwrap_or(true)
                    && madre
                        .as_ref()
                        .map(|madre| {
                            !madre.id().unwrap().eq(m.id().unwrap()) && m.is_possible_son_of(madre)
                        })
                        .unwrap_or(true)
                    && !hijos_actual.contains(m)
            })
            .collect()
    });

    create_effect(move || {
        let hijos = hijos_disponibles.get_clone();
        log(NAME, 42, &hijos);
        if hijos.len() > 0 {
            nuevo_hijo_id.set(hijos.get(0).unwrap().id().unwrap().to_owned());
        }
    });

    let options = view! {
        Keyed(
            list = hombres,
            view = |hombre|{
                let id = hombre.id().unwrap().to_owned();
                view!{
                    option(value=id){(hombre.nombre().to_owned())}
                }
            },
            key = |hombre| hombre.id().unwrap().to_owned(),
        )
    };

    let options_mujeres = view! {
        Keyed(
            list = mujeres,
            view = |mujer|{
                let id = mujer.id().unwrap().to_owned();
                view!{
                    option(value=id){(mujer.nombre().to_owned())}
                }
            },
            key = |mujer| mujer.id().unwrap().to_owned(),
        )
    };
    let hs = hombres.clone();

    let mjs = mujeres.clone();
    let select_father = move |_| {
        block_on(async move {
            log(NAME, 30, &padre_id);
            let familia = match request::<Familia>(
                format!("api/v1/families/{}", padre_id.get_clone()),
                auth.clone(),
                Method::GET,
                None::<bool>,
                true,
            )
            .await
            {
                Ok(f) => f,
                Err(_) => {
                    let mut f = Familia::default();
                    let id = padre_id.get_clone();
                    let padre = hs
                        .get_clone()
                        .iter()
                        .find(move |p| id.eq(p.id().unwrap()))
                        .cloned();
                    f.set_padre(padre.clone());
                    log(NAME, 90, &f);
                    Some(f)
                }
            };
            log(NAME, 31, &familia);
            let madre = familia
                .as_ref()
                .map(|f| f.madre().as_ref().map(|m| m.id().unwrap().clone()))
                .flatten()
                .unwrap_or_default();
            log(NAME, 85, &madre);
            familia_actual.set(familia);
            madre_id.set(madre);
        })
    };
    let select_mother = move |_| {
        block_on(async move {
            let mut familia = familia_actual.get_clone().unwrap_or_default();
            match familia.padre() {
                Some(_) => {
                    familia.set_madre(
                        mjs.get_clone()
                            .iter()
                            .find(move |m| madre_id.get_clone().eq(m.id().unwrap()))
                            .cloned(),
                    );
                }
                None => {
                    familia = match request::<Familia>(
                        format!("api/v1/families/{}", padre_id.get_clone()),
                        auth.clone(),
                        Method::GET,
                        None::<bool>,
                        true,
                    )
                    .await
                    {
                        Ok(f) => f.unwrap_or_default(),
                        Err(_) => {
                            let mut f = Familia::default();
                            let id = padre_id.get_clone();
                            let madre = mjs
                                .get_clone()
                                .iter()
                                .find(move |m| id.eq(m.id().unwrap()))
                                .cloned();
                            f.set_madre(madre.clone());
                            log(NAME, 103, &f);
                            f
                        }
                    };
                }
            }
            familia_actual.set(Some(familia));
        })
    };

    let add_hijo_fn = move |_| {
        let familia = match familia_actual.get_clone() {
            Some(mut familia) => {
                log(NAME, 160, &nuevo_hijo_id.get_clone());
                log(NAME, 161, &hijos_disponibles);
                let hijo_actual = hijos_disponibles
                    .get_clone()
                    .into_iter()
                    .find(|hijo| {
                        nuevo_hijo_id
                            .get_clone()
                            .eq(hijo.id().as_ref().unwrap().to_owned())
                    })
                    .unwrap();
                familia.add_hijo(hijo_actual);
                Some(familia.to_owned())
            }
            None => None,
        };
        familia_actual.set(familia);
    };
    create_effect(move || log(NAME, 169, &nuevo_hijo_id.get_clone()));
    view! {
        section(id="padres"){
            article(){
                label(r#for="select_hombre"){"Padre: "}
                select(name="select_hombre", on:change = select_father, bind:value=padre_id){
                    option(value=""){"-"}
                    (options)
                }
            }
            article(){
                label(r#for="select_mujer"){"Madre: "}
                select(name="select_mujer",on:change=select_mother,bind:value=madre_id){
                    option(value=""){"-"}
                    (options_mujeres)
                }
            }
        }
        section(id="hijos"){
            p(){"Hijos"}
            (familia_actual.get_clone().map(|f|{
                f.hijos().into_iter().cloned().map(|h|{
                    let id = h.id().unwrap().clone();
                    view!{
                        article(){
                            select(disabled=true){option(){(h.nombre().to_string())}}
                            button(on:click=move|_|{
                                let id = id.to_owned();
                                let familia = match familia_actual.get_clone(){
                                    Some(mut familia) => {
                                        familia.remove_hijo(id).unwrap();
                                        Some(familia)
                                    },
                                    None=>None,
                                };
                                familia_actual.set(familia);
                                // familia_actual.set_fn(move|familia_opt|{
                                //     match familia_opt.to_owned() {
                                //         Some(mut familia) => {
                                //             familia.remove_hijo(id);
                                //             Some(familia)
                                //         },
                                //         None=>None,
                                //     }
                                // });
                            }){"Borrar"}
                        }
                    }
                }).collect::<Vec<View>>()
            }).unwrap_or_default())
            article(){
                // select(bind:value=nuevo_hijo_id){
                (match hijos_disponibles.with(|hijos|hijos.len()){
                    0=>view!{},
                    _=>view!{
                        select(bind:value=nuevo_hijo_id){
                            Keyed(
                                list = hijos_disponibles,
                                view = |hijo|{
                                    let h2 = hijo.clone();
                                    view!{
                                        option(value = hijo.id().unwrap().to_owned()){(h2.nombre().to_owned())}
                                    }
                                },
                                key = |hijo| hijo.id().unwrap().to_owned(),
                            )
                        }
                        button(on:click=add_hijo_fn){"Agregar"}
                    }
                })
            }
            button(on:click=move|_|{
                block_on(async move {
                    let familia = request::<Familia>(
                        format!("api/v1/families/{}", padre_id.get_clone()),
                        auth.clone(),
                        Method::PUT,
                        familia_actual.get_clone(),
                        true,
                    )
                    .await;
                    log(NAME,255,&familia)
                })
            }){"Guardar"}
        }
    }
}
