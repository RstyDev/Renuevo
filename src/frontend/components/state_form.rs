use crate::entities::{Bautismo, Estado, Ministerio, Servicio};
use crate::frontend::{components::servicio_forms::ServicioForms, lib::log, structs::Auth};
use std::collections::HashMap;
use std::str::FromStr;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn StateForm(
    auth: Signal<Auth>,
    estado_numerado: Signal<u8>,
    estado_connector: Signal<Estado>,
) -> View {
    log("StateForm", 8, &estado_numerado.get());
    log("StateForm", 9, &estado_connector.get_clone());

    let servicios_map = create_signal(match estado_connector.get_clone() {
        Estado::Miembro { servicio, .. }
        | Estado::Diacono { servicio, .. }
        | Estado::Presbitero { servicio, .. } => servicio
            .into_iter()
            .map(|serv| (serv.area().to_string(), serv.leader().to_string()))
            .collect::<HashMap<String, String>>(),
        _ => HashMap::new(),
    });
    let map = servicios_map.clone();
    create_memo(move || log("State Form", 14, &map));
    let conversion = create_signal(match estado_connector.get_clone() {
        Estado::Miembro { conversion, .. }
        | Estado::Diacono { conversion, .. }
        | Estado::Presbitero { conversion, .. }
        | Estado::PreMiembro { conversion, .. }
        | Estado::Fundamentos { conversion, .. } => conversion.to_string(),
        _ => String::new(),
    });
    let servicios = create_signal(match estado_connector.get_clone() {
        Estado::Miembro { servicio, .. }
        | Estado::Diacono { servicio, .. }
        | Estado::Presbitero { servicio, .. } => servicio,
        _ => vec![],
    });
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
        log("StateForm", 22, &estado_connector.get_clone());
        let connector = estado_connector.get_clone();
        s1.set(match &connector {
            Estado::Miembro { servicio, .. }
            | Estado::Diacono { servicio, .. }
            | Estado::Presbitero { servicio, .. } => servicio.clone(),
            _ => vec![],
        });
        c1.set(match connector {
            Estado::Miembro { conversion, .. }
            | Estado::Diacono { conversion, .. }
            | Estado::Presbitero { conversion, .. }
            | Estado::PreMiembro { conversion, .. }
            | Estado::Fundamentos { conversion, .. } => conversion.to_string(),
            _ => String::new(),
        })
    });
    let iglesia_bautismo = create_signal(String::new());
    let fecha_bautismo = create_signal(String::new());
    let profesion_de_fe = create_signal(String::new());
    let tipo_presbitero = create_signal(String::new());

    view! {
        (match estado_numerado.get() > 2 {
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
        (match estado_numerado.get() > 0{
            true => view!{
                div(){
                    label(r#for="conversion"){"Fecha de Conversión: "}
                    input(r#type="date", name="conversion",bind:value=conversion){}
                }
                div(){
                    label(r#for="iglesia_bautismo"){"Iglesia de Bautismo: "}
                    input(name="iglesia_bautismo", bind:value=iglesia_bautismo){}
                }
                div(){
                    label(r#for="fecha_bautismo"){"Fecha de Bautismo: "}
                    input(r#type="date",name="fecha_bautismo", bind:value=fecha_bautismo){}
                }
                div(){
                    label(r#for="profesion_de_fe"){"Profesión de Fe: "}
                    input(name="profesion_de_fe", bind:value=profesion_de_fe){}
                }

            },
            false => view!{},
        })
        (match estado_numerado.get() > 1 {
            true => view!{ServicioForms(servicios = servicios, servicios_map = servicios_map)},
            false => view!{},
        })
    }
}
/*
    Visitante,
    Nuevo,
    Fundamentos {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    PreMiembro {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    Miembro {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Diacono {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Persbitero {
        tipo: TipoPresbitero,
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
*/
