use std::collections::HashMap;
use sycamore::prelude::*;
use crate::{entities::{Servicio, Ministerio}, frontend::lib::log};

#[component(inline_props)]
pub fn ServicioForms(servicios: Signal<Vec<Servicio>>) -> View {
    let servicios_signal = create_signal(HashMap::<String,bool>::new());

    let servicio = create_signal(String::new());
    let lead = create_signal(false);
    let new_serv_area = create_signal(String::new());
    let new_lead = create_signal(String::new());
    // let update_leader = move |area:Signal<String>,checked:Signal<String>|{
    //     servicios.set_fn(|servicios|{
    //         let mut servicios = servicios.clone();
    //         let i = servicios.iter().enumerate().find_map(|(i,servicio)|match (servicio.area(),area.get_clone().as_str()){
    //             (Ministerio::Sonido, "Sonido") => Some(i),
    //             (Ministerio::Palabra, "Palabra") => Some(i),
    //             (Ministerio::Tesoro, "Tesoro") => Some(i),
    //             (Ministerio::Bienvenida, "Bienvenida") => Some(i),
    //             (Ministerio::Misericordia, "Misericordia") => Some(i),
    //             (Ministerio::Alabanza, "Alabanza") => Some(i),
    //             (Ministerio::Letras, "Letras") => Some(i),
    //             (Ministerio::Presbiterado, "Presbiterado") => Some(i),
    //             (Ministerio::Redes, "Redes") => Some(i),
    //             _ => None,
    //         }).unwrap();
    //         let mut servicio = servicios.remove(i);
    //         servicio.set_leader(checked.get_clone().parse<bool>());
    //         servicios.insert(i,servicio);
    //         servicios
    //     });
    //
    // };



    view!{
        section(id="servicio_forms"){
            Keyed(
                list = servicios,
                view=|serv|{
                    // let area = serv.area().to_string();
                    let (serv1,serv2) = (serv.clone(),serv.clone());
                    let checked = create_signal(serv.leader().to_string());
                    create_effect(move ||{log("Servicio Forms",21,&checked.get_clone())});
                    view!{
                        article(){
                            label(r#for="area_servicio"){"Area de Servicio: "}
                            select(name="area_servicio", value=serv.area().to_string(), disabled = true){
                                option(value=serv1.area().to_string()){(serv2.area().to_string())}
                            }
                            select(name="leader", bind:value=checked, on:change=move |_|{
                                log("ServicioForms",29,&checked.get_clone());
                            }){
                                option(value="true"){"Leader"}
                                option(value="false"){"No Leader"}
                            }
                            button(){"Quitar Servicio"}
                        }
                    }
                },
                key=|serv|serv.area().to_string(),
            )
            article(){
                label(r#for="area_servicio"){"Area de Servicio: "}
                select(name="area_servicio", bind:value=new_serv_area){
                    option(value="Sonido"){"Sonido"}
                    option(value="Tesoro"){"Tesoro"}
                    option(value="Letras"){"Letras"}
                    option(value="Bienvenida"){"Bienvenida"}
                    option(value="Redes"){"Redes"}
                    option(value="Alabanza"){"Alabanza"}
                    option(value="Misericordia"){"Misericordia"}
                    option(value="Palabra"){"Palabra"}
                    option(value="Presbiterado"){"Presbiterado"}
                }

                label(r#for="leader"){"Leader: "}
                input(r#type="checkbox",name="leader", bind:value=new_lead)
                button(){"Agregar Servicio"}
            }
        }
    }
}