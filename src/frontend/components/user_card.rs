use crate::entities::{Estado, Persona};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn UserCard(user: Persona, mode: Mode, action: Signal<ActionOnUser>) -> View {
    let expanded = create_signal(false);
    let (us1, us2) = (user.clone(), user.clone());

    let servicio_selector = create_selector(move || match us2.estado() {
        Estado::Anciano { servicio, .. }
        | Estado::Diacono { servicio, .. }
        | Estado::Miembro { servicio, .. } => {
            let mut servicios = String::from(&servicio[0].area().to_string().to_lowercase());
            for i in 1..servicio.len() {
                servicios.push_str(", ");
                servicios.push_str(servicio[i].area().to_string().to_lowercase().as_str());
            }
            Some(servicios)
        }
        _ => None,
    });
    view! {
        article(class = "user_card",on:click = move |_|{expanded.set(!expanded.get())}){
            h4(){
                (format!("{} {}",user.nombre(),user.apellido()))
            }
            section(class=format!("user_card_info_{}", expanded.get())){
                p(){(format!("Estado: {}",us1.estado().to_plain_string().to_lowercase()))}
                (match servicio_selector.get_clone(){
                    Some(servicios) => view!{p(){(format!("Servicio: {}",servicios))}},
                    None => view!{},
                })
                (match mode {
                    Mode::View => view!{},
                    Mode::Edit => view!{
                        button(on:click=move|_|{
                            action.set(ActionOnUser::Edit);
                        }){"Edit"}
                    },
                    Mode::Delete => view!{
                        button(on:click=move|_|{
                            action.set(ActionOnUser::Delete);
                        }){"Delete"}
                    },
                })
            }
            // (match expanded_selector.get(){
            //     true => {
            //         let us1 = us1.to_owned();
            //         view!{
            //             section(class="user_card_info"){
            //                 p(){(format!("Estado: {}",us1.estado().to_plain_string().to_lowercase()))}
            //                 (match servicio_selector.get_clone(){
            //                     Some(servicios) => view!{p(){(format!("Servicio: {}",servicios))}},
            //                     None => view!{},
            //                 })
            //             }
            //         }
            //     },
            //     false=> view!{},
            // })

        }
    }
}

pub enum Mode {
    View,
    Edit,
    Delete,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ActionOnUser {
    None,
    Edit,
    Delete,
}
