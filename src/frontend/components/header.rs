use crate::entities::{Estado, Persona};
use crate::frontend::lib::log;
use crate::frontend::structs::{Auth, Tabs};
use sycamore::prelude::*;
use sycamore::reactive::Signal;
const NAME: &'static str = "Header";

#[component(inline_props)]
pub fn Header(auth: Signal<Auth>, tabs: Signal<Tabs>, hermano: Signal<Option<Persona>>) -> View {
    let show_menu = create_signal(false);
    let sh_menu = show_menu.clone();
    let show_ministerio = create_signal(false);
    let show_user = create_signal(false);
    let show_nosotros = create_signal(false);
    let set_login_tab = move |_| tabs.set(Tabs::Login);
    let auth_selector = create_selector(move || auth.get_clone());

    let ministerio_selector = create_selector(move || {
        hermano
            .get_clone()
            .map(|h| match h.estado() {
                Estado::Visitante => None,
                Estado::Nuevo => None,
                Estado::Fundamentos { .. } => None,
                Estado::PreMiembro { .. } => None,
                Estado::Miembro { servicio, .. } => Some(servicio.to_owned()),
                Estado::Diacono { servicio, .. } => Some(servicio.to_owned()),
                Estado::Presbitero { servicio, .. } => Some(servicio.to_owned()),
            })
            .flatten()
    });
    let (sh1, sh2, sh3) = (
        show_menu.clone(),
        show_ministerio.clone(),
        show_user.clone(),
    );
    let hide_all_dropdowns = move || {
        sh1.set_silent(false);
        sh2.set_silent(false);
        sh3.set_silent(false);
    };
    create_memo(move || match auth.get_clone() {
        Auth::NotLogged => hide_all_dropdowns(),
        _ => (),
    });

    view! {
        header(){
            img(id="logo", title="logo", src="public/RENUEVO.png"){}
            section(id="menu_burger"){
                a(id="burger", on:click=move |_|{show_menu.set(true)}){
                    i(class="fa fa-bars"){}
                }
            }
            section(){
                (match auth_selector.get_clone(){
                    Auth::Logged(_) => view!{
                        a(id="user",on:click = move |_| show_user.set(true)){
                            i(class="fa fa-user-circle"){}
                        }
                        div(id="user-dropdown",class=format!("modal-user {}", show_user.get().to_string()),on:click=move |_|{
                                show_user.set(false);
                            }) {
                                div(class="modal-content-user"){
                                    ul(id="dropdown_user"){
                                        li(on:click = move |_| tabs.set(Tabs::PasswordChange)){a(){"Profile"}}
                                        li(on:click = move |_| {
                                            auth.set(Auth::NotLogged);
                                    }){a(){"Salir"}}
                                    }
                                }
                            }
                    },
                    Auth::NotLogged => view!{
                        a(id="user",class="out",on:click = set_login_tab){"Acceso"}
                    },
                })
            }
            div(id="menu",class=format!("modal {}", sh_menu.get().to_string()),on:click=move |_|{
                sh_menu.set(false);
            }){
                div(class="modal-content"){
                    ul(){
                        li(on:click=move |_|{tabs.set(Tabs::Inicio)}){a(){"Inicio"}}
                        li(on:click=move |_|{show_nosotros.set(true)}){a(){"Nosotros"}}
                        div(id="dropdown",class=format!("modal-nosotros {}", show_nosotros.get().to_string()),on:click=move |_|{
                            show_nosotros.set(false);
                        }) {
                            div(class="modal-content-nosotros"){
                                ul(id="dropdown_nosotros"){
                                    li(){a(on:click=move |_|{tabs.set(Tabs::QuienesSomos)}){"Quiénes somos"}}
                                    li(){a(){"Confesión de fe"}}
                                    li(){a(){"Membresía"}}
                                }
                            }
                        }
                        li(){a(href="https://www.paypal.com/ncp/payment/54LQSZXQDVTR4",target="_blank"){"Involúcrate"}}
                        (match auth_selector.get_clone(){
                            Auth::Logged(_) => view!{
                                li(on:click=move |_|{tabs.set(Tabs::Miembros)}){a(){"Miembros"}}
                            },
                            Auth::NotLogged => view!{},
                        })
                        (match ministerio_selector.get_clone(){
                        None=> view!{},
                        Some(_)=> view! {
                            li(on:click=move |_|{
                                    sh_menu.set(false);
                                show_ministerio.set(true)
                            }){
                                    a(){"Ministerio"}}
                                div(id="dropdown",class=format!("modal-ministerio {}", show_ministerio.get().to_string()),on:click=move |_|{
                                    show_ministerio.set(false);
                                }) {
                                    div(class="modal-content-ministerio"){
                                        ul(id="dropdown_ministerio"){
                                            (match ministerio_selector.get_clone(){
                                                None=>vec![view!{}],
                                                Some(servicio)=> servicio.iter().cloned().map(|s| {
                                                    let s2 = s.clone();
                                                    view! {
                                                        li(on:click=move |_|{
                                                            log(NAME,109,&s);
                                                            tabs.set(Tabs::Ministerio(s.area().clone()));
                                                        }){
                                                            a(){(s2.area().to_string())}}}
                                                }).collect::<Vec<View>>(),
                                            })
                                        }
                                    }
                                }
                            },
                        })
                    }
                }
            }
        }
    }
}
