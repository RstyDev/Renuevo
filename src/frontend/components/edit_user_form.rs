use async_std::task::block_on;
use reqwest::Method;
use crate::entities::{Estado, EstadoCivil, Persona, Sexo};
use crate::frontend::components::{
    state_form::StateForm,
    user_card::{ActionOnUser, Mode},
    user_cards::UserCards,
};
use crate::frontend::{lib::{log, refresh_users, request}, structs::Auth};
use sycamore::prelude::*;

const NAME: &'static str = "Edit User Form";


#[component(inline_props)]
pub fn EditUserForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let form = create_signal(None::<Persona>);
    let action = create_signal(ActionOnUser::None);
    let act2 = action.clone();
    let estado_civil = create_signal(String::new());
    let estado = create_signal(form.get_clone().map(|user|{
        user.estado().to_plain_string()
    }).unwrap_or_default());
    let form_selector = create_selector(move || form.get_clone());
    let opciones_estado = create_signal(0);
    let estado_connector = create_signal(Estado::Visitante);
    let updated_estado = create_signal(false);
    create_memo(move || match act2.get_clone() {
        ActionOnUser::Edit(persona) => {
            opciones_estado.set_silent(match persona.estado() {
                Estado::Visitante => 0,
                Estado::Nuevo => 1,
                Estado::Fundamentos { .. } => 2,
                Estado::PreMiembro { .. } => 3,
                Estado::Miembro { .. } => 4,
                Estado::Diacono { .. } => 5,
                Estado::Presbitero { .. } => 6,
            });
            estado_civil.set_silent(persona.estado_civil().to_string());
            estado.set_silent(persona.estado().to_plain_string());
            estado_connector.set_silent(persona.estado().clone());
            form.set(Some(persona.to_owned()));
            act2.set_silent(ActionOnUser::None)
        }
        _ => (),
    });
    create_effect(move || {
        if let Some(user) = form.get_clone(){
            log(NAME,63,&user.estado());
            estado.set(user.estado().to_plain_string())
        }
    });
    create_effect(move || {
        log(NAME,68,&estado.get_clone());
    });

    create_memo(move || {
        if updated_estado.get(){
            let estado = estado_connector.get_clone();
            match form.get_clone_untracked(){
                Some(persona) => {
                    let user = Persona::new(
                        persona.id().cloned(),
                        persona.password().cloned(),
                        persona.nombre().to_string(),
                        persona.apellido().to_string(),
                        persona.sexo().clone(),
                        persona.nacimiento().clone(),
                        EstadoCivil::from_string(estado_civil.get_clone()).unwrap(),
                        estado,
                        persona.registrado().clone(),
                    );
                    log(NAME,81,&user);
                    block_on(async move {
                        log(NAME,86,&user.id());
                        let result = request::<Persona>(format!("api/v1/users/{}",user.id().unwrap()), auth.clone(), Method::PUT, Some(user), true)
                            .await;
                        log(NAME,90,&result);
                        // nombre.set(String::new());
                        // apellido.set(String::new());
                        // estado_civil.set(String::new());
                        // nacimiento.set(String::new());
                        refresh_users(miembros.clone(), auth.clone()).await;
                    });
                },
                None => (),
            }
            updated_estado.set(false);
        }
        // log(NAME,66,&estado_connector.get_clone());
    });

    create_effect(move || {
        opciones_estado.set(match estado.get_clone().as_str() {
            "Visitante"=> 0,
            "Nuevo" => 1,
            "Fundamentos" => 2,
            "PreMiembro" => 3,
            "Miembro" => 4,
            "Diacono" => 5,
            "Presbitero" => 6,
            _ => 0,
        });
        log(NAME,61,&estado.get_clone());
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
                                            option(value = "Presbitero"){"Presbítero"}
                                        },
                                    },
                                    Estado::Diacono {..} | Estado::Presbitero {..} => view!{
                                        option(value = "Visitante"){"Visitante"}
                                        option(value = "Nuevo"){"Nuevo"}
                                        option(value = "Fundamentos"){"Fundamentos"}
                                        option(value = "PreMiembro"){"Pre Miembro"}
                                        option(value = "Miembro"){"Miembro"}
                                        option(value = "Diacono"){"Diácono"}
                                        option(value = "Presbitero"){"Presbítero"}
                                    }
                                })
                            }
                        }
                        StateForm(estado_numerado = opciones_estado, estado_connector = estado_connector, updated_estado = updated_estado)
                        //aca se sigue
                    }
                }
            },
            None => view!{},
        })
        UserCards(auth = auth, miembros = miembros, mode = mode)
    }
}