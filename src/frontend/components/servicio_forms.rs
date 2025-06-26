use std::collections::HashMap;
use sycamore::prelude::*;
use crate::{entities::Servicio, frontend::lib::log};

#[component(inline_props)]
pub fn ServicioForms(servicios: Signal<Vec<Servicio>>) -> View {
    let servicios_signal = create_signal(HashMap::<String,bool>::new());

    let servicio = create_signal(String::new());
    let lead = create_signal(false);
    let new_serv_area = create_signal(String::new());
    let new_lead = create_signal(String::new());
    view!{
        section(id="servicio_forms"){
            Keyed(
                list = servicios,
                view=|serv|{
                    // let area = serv.area().to_string();
                    let (serv1,serv2) = (serv.clone(),serv.clone());
                    let checked = create_signal(serv.leader().to_string());
                    view!{
                        article(){
                            label(r#for="area_servicio"){"Area de Servicio: "}
                            select(name="area_servicio", value=serv.area().to_string(), disabled = true){
                                option(value=serv1.area().to_string()){(serv2.area().to_string())}
                            }
                            label(r#for="leader"){"Leader: "}
                            input(r#type="checkbox",name="leader", bind:value=checked, on:input=move |_|{
                                log("ServicioForms",29,&checked.get_clone());
                            })
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