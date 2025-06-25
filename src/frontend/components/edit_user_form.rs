use crate::entities::{Estado, Persona};
use crate::frontend::components::{user_card::{ActionOnUser, Mode, UserCard}, user_cards::UserCards};
use crate::frontend::{lib::{request,log}, structs::Auth};
use async_std::task::block_on;
use chrono::NaiveDate;
use reqwest::Method;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn EditUserForm(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let form = create_signal(None::<Persona>);
    let action = create_signal(ActionOnUser::None);
    let act2 = action.clone();
    let estado_civil = create_signal(String::new());
    let estado = create_signal(String::new());
    let conversion = create_signal(String::new());
    let fecha_bautismo = create_signal(String::new());
    let profesion_de_fe = create_signal(String::new());
    let iglesia_bautismo = create_signal(String::new());
    let estado_selector = create_selector(move ||match form.get_clone(){
        Some(user) => user.estado().clone(),
        None => Estado::Visitante,
    });

    create_memo(move || {
        match act2.get_clone() {
            ActionOnUser::Edit(persona) => {
                form.set(Some(persona.to_owned()));
                act2.set_silent(ActionOnUser::None)
            },
            _=>(),
        }
    });
    create_effect(move || {
        log("Edit ",36,&estado_selector.get_clone());
    });
    create_effect(move|| {
        log("Edit User Form",24,&form.get_clone());
        match form.get_clone() {
            Some(user) => {
                estado_civil.set(user.estado_civil().to_string());
                estado.set(user.estado().to_plain_string().to_string());
            },
            None => (),
        }
    });

    let mode = Mode::Edit(action.clone());

    view! {
        (match form.get_clone(){
            Some(user) => {
                let (user2,user3) = (user.clone(), user.clone());
                view!{
                    form(){
                        input(name="nombre",value = user.nombre().to_string(), disabled = true){}
                        input(name="apellido", value = user2.apellido().to_string(), disabled = true ){}
                        input(r#type="date", name = "nacimiento", value = user3.nacimiento().to_string(), disabled = true ){}
                        select(name = "estado_civil", bind:value = estado_civil) {
                            option(value="Soltero"){"Soltero/a"}
                            option(value="Casado"){"Casado/a"}
                            option(value="Viudo"){"Viudo/a"}
                        }
                        select(name = "estado", bind:value = estado) {
                            (match estado_selector.get_clone(){
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
                                _ => view!{
                                    option(value = "Visitante"){"Visitante"}
                                    option(value = "Nuevo"){"Nuevo"}
                                    option(value = "Fundamentos"){"Fundamentos"}
                                    option(value = "PreMiembro"){"Pre Miembro"}
                                    option(value = "Miembro"){"Miembro"}
                                    option(value = "Diacono"){"Diácono"}
                                },
                            })
                        }

                    }
                }
            },
            None => view!{},
        })
        UserCards(auth = auth, miembros = miembros, mode = mode)
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
    Anciano {
        tipo: TipoAnciano,
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
*/
/*
    id: Option<String>,
    password: Option<String>,
    nombre: String,
    apellido: String,
    nacimiento: NaiveDate,
    estado_civil: EstadoCivil,
    estado: Estado,
    registrado: NaiveDate,
*/