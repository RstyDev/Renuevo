use crate::entities::{Bautismo, Estado, Ministerio, Servicio, TipoPresbitero};
use crate::frontend::{components::servicio_forms::ServicioForms, lib::log};
use std::collections::HashMap;
use sycamore::prelude::*;
use web_sys::MouseEvent;
const NAME: &'static str = "State Form";

fn get_all_signals(estado: Signal<Estado>)->(Signal<HashMap<String,String>>,Signal<Vec<Servicio>>,Signal<String>,Signal<String>,Signal<String>,Signal<String>,Signal<String>){
    let estado = estado.get_clone();
    let servicios_map = create_signal(estado.get_servicio().map(|servicio|servicio.into_iter()              //uno
        .map(|serv| (serv.area().to_string(), serv.leader().to_string()))
        .collect::<HashMap<String, String>>()).unwrap_or_default());
    let servicios = create_signal(estado.get_servicio().cloned().unwrap_or_default());                              //tres
    let conversion = create_signal(estado.get_conversion().map(|c|c.to_string()).unwrap_or_default());        //dos
    let iglesia_bautismo = create_signal(estado.get_bautismo().map(|b|b.iglesia().to_string()).unwrap_or_default());    //cuatro
    let fecha_bautismo = create_signal(estado.get_bautismo().map(|b|b.fecha().to_string()).unwrap_or_default()); //cinco
    let profesion_de_fe = create_signal(estado.get_bautismo().map(|b|b.profesion_de_fe().map(|p|p.to_string())).flatten().unwrap_or_default()); //6
    let tipo_presbitero = create_signal(estado.get_tipo_presbitero().map(|p|p.to_string()).unwrap_or_default());//7
    (servicios_map,servicios,conversion,iglesia_bautismo,fecha_bautismo,profesion_de_fe,tipo_presbitero)
}

#[component(inline_props)]
pub fn StateForm(
    estado_numerado: Signal<u8>,
    estado_connector: Signal<Estado>,
    updated_estado: Signal<bool>,
) -> View {
    // log(NAME, 8, &estado_numerado.get());
    // log(NAME, 9, &estado_connector.get_clone());

    let (servicios_map,servicios,conversion,iglesia_bautismo,fecha_bautismo,profesion_de_fe,tipo_presbitero) = get_all_signals(estado_connector);
    let bautismo_disabled = create_signal(match estado_connector.get_clone(){
        Estado::Nuevo => false,
        Estado::Fundamentos { bautismo, ..} | Estado::PreMiembro { bautismo, ..} => match bautismo.map(|b|b.profesion_de_fe()).flatten() {
            Some(_) => true,
            None => false,
        },
        _ => true,
    });

    log(NAME,41,&bautismo_disabled.get());

    let map = servicios_map.clone();
    create_memo(move || log(NAME, 14, &map));

    let c1 = conversion.clone();
    let (s1, s2) = (servicios.clone(), servicios.clone());
    create_memo(move || {
        s2.set(
            map.get_clone()
                .into_iter()
                .map(|(k, v)| {
                    Servicio::new(
                        v.parse::<bool>().unwrap(),
                        match k.as_str() {
                            "Sonido" => Ministerio::Sonido,
                            "Tesoro" => Ministerio::Tesoro,
                            "Letras" => Ministerio::Letras,
                            "Bienvenida" => Ministerio::Bienvenida,
                            "Redes" => Ministerio::Redes,
                            "Alabanza" => Ministerio::Alabanza,
                            "Misericordia" => Ministerio::Misericordia,
                            "Palabra" => Ministerio::Palabra,
                            "Presbiterado" => Ministerio::Presbiterado,
                            _ => panic!("Not possible"),
                        },
                    )
                })
                .collect::<Vec<Servicio>>(),
        )
    });
    create_memo(move || {
        log(NAME, 22, &estado_connector.get_clone());
        let connector = estado_connector.get_clone();
        s1.set(connector.get_servicio().cloned().unwrap_or_default());
        c1.set(connector.get_conversion().map(|c|c.to_string()).unwrap_or_default());
    });

    let get_bautismo = move || -> Option<Bautismo> {
        let fecha = fecha_bautismo.get_clone();
        let profesion = profesion_de_fe.get_clone();
        let iglesia = iglesia_bautismo.get_clone();
        log(NAME,83,&format!("fecha: {}, profesion: {}, iglesia: {}",fecha.len(),profesion.len(),iglesia.len()));
        match fecha.len()>0{
            true => Some(Bautismo::new(
                fecha.parse().unwrap(),
                match profesion.len()>0{
                    true => Some(profesion.parse().unwrap()),
                    false => None,
                },
                iglesia.to_owned(),
            )),
            false => None,
        }
    };
    let save_event = move |ev: MouseEvent| {
        ev.prevent_default();
        estado_connector.set(
            match estado_numerado.get(){
                0 => Estado::Visitante,
                1 => Estado::Nuevo,
                2 => Estado::Fundamentos {
                    bautismo: get_bautismo(),
                    conversion: conversion.get_clone().parse().unwrap(),
                },
                3 => Estado::PreMiembro {
                    bautismo: get_bautismo(),
                    conversion: conversion.get_clone().parse().unwrap(),
                },
                4 => Estado::Miembro {
                    servicio: servicios.get_clone(),
                    bautismo: get_bautismo().unwrap(),
                    conversion: conversion.get_clone().parse().unwrap(),
                },
                5 => Estado::Diacono {
                    servicio: servicios.get_clone(),
                    bautismo: get_bautismo().unwrap(),
                    conversion: conversion.get_clone().parse().unwrap(),
                },
                6 => Estado::Presbitero {
                    tipo: match tipo_presbitero.get_clone().as_str() {
                        "Maestro" => TipoPresbitero::Maestro,
                        "Governante" => TipoPresbitero::Governante,
                        _ => panic!("Not possible"),
                    },
                    servicio: servicios.get_clone(),
                    bautismo: get_bautismo().unwrap(),
                    conversion: conversion.get_clone().parse().unwrap(),
                },
                _=> panic!("Not possible"),
            }
        );
        updated_estado.set(true);
    };
    view! {
        (match estado_numerado.get() > 5 {
            true => view!{
                div(){
                    label(r#for="tipo_presbitero"){"Tipo de Presbítero"}
                    select(name="tipo_presbitero", bind:value=tipo_presbitero){
                        option(value="Governante"){"Governante"}
                        option(value="Maestro"){"Maestro"}
                    }
                }
            },
            false => view!{},
        })
        (match estado_numerado.get() > 1 {
            true => view!{
                div(){
                    label(r#for="conversion"){"Fecha de Conversión: "}
                    input(r#type="date", name="conversion",bind:value=conversion, disabled = bautismo_disabled.get()){}
                }
                div(){
                    label(r#for="iglesia_bautismo"){"Iglesia de Bautismo: "}
                    input(name="iglesia_bautismo", bind:value=iglesia_bautismo, disabled = bautismo_disabled.get()){}
                }
                div(){
                    label(r#for="fecha_bautismo"){"Fecha de Bautismo: "}
                    input(r#type="date",name="fecha_bautismo", bind:value=fecha_bautismo, disabled = bautismo_disabled.get()){}
                }
                div(){
                    label(r#for="profesion_de_fe"){"Profesión de Fe: "}
                    input(r#type="date",name="profesion_de_fe", bind:value=profesion_de_fe, disabled = bautismo_disabled.get()){}
                }

            },
            false => view!{},
        })
        (match estado_numerado.get() > 3 {
            true => view!{ServicioForms(servicios = servicios, servicios_map = servicios_map)},
            false => view!{},
        })
        button(on:click=save_event){"Guardar"}
    }
}
