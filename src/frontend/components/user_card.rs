use crate::entities::{Estado, Persona};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn UserCard(user: Persona, mode: Mode, action: Signal<ActionOnUser>) -> View {
    let expanded = create_signal(false);
    let (us1, us2) = (user.clone(), user.clone());

    let servicio_selector = create_selector(move || match us2.estado() {
        Estado::Presbitero { servicio, .. }
        | Estado::Diacono { servicio, .. }
        | Estado::Miembro { servicio, .. } => {
            let mut servicios = match servicio.len()>0{
                true => servicio[0].area().to_string(),
                _ => "".to_string(),
            };
            // let mut servicios = String::from(&servicio[0].area().to_string());
            for i in 1..servicio.len() {
                servicios.push_str(", ");
                servicios.push_str(servicio[i].area().to_string().as_str());
            }
            Some(servicios)
        }
        _ => None,
    });
    let user3 = user.clone();
    view! {
        article(class = "user_card",on:click = move |_|{expanded.set(!expanded.get())}){
            h4(){
                (format!("{} {}",user.nombre(),user.apellido()))
            }
            section(class=format!("user_card_info_{}", expanded.get())){
                p(){(format!("Estado: {}",us1.estado().to_plain_string()))}
                (match servicio_selector.get_clone(){
                    Some(servicios) => view!{p(){(format!("Servicio: {}",servicios))}},
                    None => view!{},
                })
                (match mode {
                    Mode::View => view!{},
                    Mode::Edit(act) => {
                        let user2 = user3.clone();
                        view!{
                            button(on:click=move|_|{
                                act.set(ActionOnUser::Edit(user2.to_owned()));
                            }){"Edit"}
                        }
                    },
                    Mode::Delete => view!{
                        button(on:click=move|_|{
                            action.set(ActionOnUser::Delete);
                        }){"Delete"}
                    },
                })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    View,
    Edit(Signal<ActionOnUser>),
    Delete,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ActionOnUser {
    None,
    Edit(Persona),
    Delete,
}
