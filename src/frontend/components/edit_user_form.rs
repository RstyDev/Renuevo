use crate::entities::{Estado, Persona, Sexo};
use crate::frontend::components::{user_card::{ActionOnUser, Mode}, user_cards::UserCards};
use crate::frontend::{lib::log, structs::Auth};
use sycamore::prelude::*;


#[component(inline_props)]
pub fn EditUserForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let form = create_signal(None::<Persona>);
    let action = create_signal(ActionOnUser::None);
    let act2 = action.clone();
    let estado_civil = create_signal(String::new());
    let estado = create_signal(String::new());
    let conversion = create_signal(String::new());
    let fecha_bautismo = create_signal(String::new());
    let profesion_de_fe = create_signal(String::new());
    let iglesia_bautismo = create_signal(String::new());
    let form_selector = create_selector(move ||form.get_clone());
    // let estado_selector = create_selector(move ||
    //     EstadoLocal::from_str(estado.get_clone().as_str())
    // );
    // create_effect(move || {
    //
    // });

    // create_effect(move || {
    //     log("Edit:",27,&estado_selector.get_clone())
    // });

    create_memo(move || {
        match act2.get_clone() {
            ActionOnUser::Edit(persona) => {
                form.set(Some(persona.to_owned()));
                act2.set_silent(ActionOnUser::None)
            },
            _=>(),
        }
    });
    // create_effect(move || {
    //     log("Edit ",36,&estado_selector.get_clone());
    // });
    create_effect(move|| {
        log("Edit User Form",24,&form.get_clone());
        match form.get_clone() {
            Some(user) => {
                estado.set(user.estado().to_plain_string().to_string());
                estado_civil.set(user.estado_civil().to_string());
                log("Edit User Form",41,&user.estado().to_plain_string());
            },
            None => (),
        }
    });


    let mode = Mode::Edit(action.clone());


    view! {
        (match form_selector.get_clone(){
            Some(user) => {
                let (u2,u3,u4,u5,u6) = (user.clone(), user.clone(), user.clone(),user.clone(),user.clone());
                view!{
                    form(){
                        div(){
                            label(r#for="nombre"){"Nombre: "}
                            input(name="nombre",value = user.nombre().to_string(), disabled = true){}
                        }
                        div(){
                            label(r#for="apellido"){"Apellido: "}
                            input(name="apellido", value = u2.apellido().to_string(), disabled = true ){}
                        }
                        div(){
                            label(r#for="nacimiento"){"Nacimiento: "}
                            input(r#type="date", name = "nacimiento", value = u3.nacimiento().to_string(), disabled = true ){}
                        }
                        div(){
                            label(r#for="sexo"){"Sexo: "}
                            select(name="sexo", value = u5.sexo().to_string(), disabled = true) {
                                option(value=""){
                                    (u6.sexo().to_string())
                                }
                            }
                        }
                        div(){
                            label(r#for="estado_civil"){"Estado Civil: "}
                            select(name = "estado_civil", bind:value = estado_civil) {
                                option(value="Soltero"){"Soltero/a"}
                                option(value="Casado"){"Casado/a"}
                                option(value="Viudo"){"Viudo/a"}
                            }
                        }
                        div(){
                            label(r#for="estado"){"Estado: "}
                            select(name = "estado", bind:value = estado) {
                                (match u4.estado(){
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
                                    Estado::Miembro {..} => match u4.sexo() {
                                        Sexo::Femenino => view!{
                                            option(value = "Visitante"){"Visitante"}
                                            option(value = "Nuevo"){"Nuevo"}
                                            option(value = "Fundamentos"){"Fundamentos"}
                                            option(value = "PreMiembro"){"Pre Miembro"}
                                            option(value = "Miembro"){"Miembro"}
                                        },
                                        Sexo::Masculino => view!{
                                            option(value = "Visitante"){"Visitante"}
                                            option(value = "Nuevo"){"Nuevo"}
                                            option(value = "Fundamentos"){"Fundamentos"}
                                            option(value = "PreMiembro"){"Pre Miembro"}
                                            option(value = "Miembro"){"Miembro"}
                                            option(value = "Diacono"){"Diácono"}
                                            option(value = "Anciano"){"Anciano"}
                                        },
                                    },
                                    Estado::Diacono {..} | Estado::Anciano {..} => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                        option(value = "PreMiembro"){"Pre Miembro"}
                                        option(value = "Miembro"){"Miembro"}
                                        option(value = "Diacono"){"Diácono"}
                                        option(value = "Anciano"){"Anciano"}
                                    }
                                })
                            }
                        }
                        // StateForm(auth = auth.clone() )
                        //aca se sigue
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