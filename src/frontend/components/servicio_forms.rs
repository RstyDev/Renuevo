use crate::{entities::Servicio, frontend::lib::log};
use std::collections::HashMap;
use sycamore::prelude::*;
use web_sys::MouseEvent;
const NAME: &'static str = "Servicio Forms";

#[component(inline_props)]
pub fn ServicioForms(
    servicios: Signal<Vec<Servicio>>,
    servicios_map: Signal<HashMap<String, String>>,
) -> View {
    let new_serv_area = create_signal(String::new());
    let new_lead = create_signal(String::new());
    let (map, map2) = (servicios_map.clone(), servicios_map.clone());
    let update_leader = move |area: String, checked: String| {
        servicios_map.set_fn(|map| {
            let mut map = map.clone();
            map.insert(area.to_owned(), checked.to_owned());
            map
        })
    };

    let opciones_servicio = create_signal({
        let map = map2.get_clone();
        vec![
            "Sonido",
            "Tesoro",
            "Letras",
            "Bienvenida",
            "Redes",
            "Alabanza",
            "Misericordia",
            "Palabra",
            "Presbiterado",
        ]
        .into_iter()
        .filter(move |opt| (!map.contains_key(*opt)))
        .collect::<Vec<&str>>()
    });
    let opciones_servicio_selector =
        create_selector(move || opciones_servicio.with(|opts| opts.len() > 0));

    view! {
        section(id="servicio_forms"){
            Keyed(
                list = servicios,
                view=move |serv|{
                    // let area = serv.area().to_string();
                    let (serv1,serv2,serv3,serv4) = (serv.clone(),serv.clone(),serv.clone(),serv.clone());

                    let checked = create_signal(serv.leader().to_string());
                    create_effect(move ||{log(NAME,21,&checked.get_clone())});
                    view!{
                        article(){
                            label(r#for="area_servicio"){"Area de Servicio: "}
                            select(name="area_servicio", value=serv.area().to_string(), disabled = true){
                                option(value=serv1.area().to_string()){(serv2.area().to_string())}
                            }
                            select(name="leader", bind:value=checked, on:change=move |_|{
                                update_leader(serv3.area().to_string(),checked.get_clone())
                            }){
                                option(value="true"){"Leader"}
                                option(value="false"){"No Leader"}
                            }
                            button(on:click=move|ev:MouseEvent|{
                                ev.prevent_default();
                                map.set_fn(|map|{
                                    let mut map = map.clone();
                                    map.remove(&serv4.area().to_string());
                                    map
                                });
                            }){"Quitar Servicio"}
                        }
                    }
                },
                key=|serv|serv.area().to_string(),
            )
            (match opciones_servicio_selector.get(){
                true=>view!{
                    article(){
                        label(r#for="area_servicio"){"Area de Servicio: "}
                        select(name="area_servicio", bind:value=new_serv_area){
                            Keyed(
                                list=opciones_servicio,
                                view=|opt|view!{
                                    option(value=opt.to_owned()){(opt.to_owned())}
                                },
                                key=|opt|opt.to_owned()
                            )
                        }
                        label(r#for="leader"){"Leader: "}
                        select(name="leader", bind:value=new_lead){
                            option(value="true"){"Leader"}
                            option(value="false"){"No Leader"}
                        }
                        button(on:click=move|ev:MouseEvent|{
                            ev.prevent_default();
                            let area = new_serv_area.get_clone();
                            if area.len()>0 {
                                update_leader(new_serv_area.get_clone(),new_lead.get_clone());
                                opciones_servicio.set_fn(|opts|{
                                    let mut opts = opts.clone();
                                    if let Some(i) = opts.iter().enumerate().find_map(|(i,&opt)|{
                                        opt.eq(&new_serv_area.get_clone()).then_some(i)
                                    }){
                                        opts.remove(i);
                                    }
                                    opts
                                });
                                new_serv_area.set(String::new())
                            }
                        }){"Agregar Servicio"}
                    }
                },
                false=>view!{}
            })
        }
    }
}
