use crate::entities::Persona;
use crate::frontend::{
    components::footer::Footer,
    lib::{log, request},
    pages::{
        login::Login, miembros::Miembros, ministerio::Ministerio, quienes_somos::QuienesSomos,
    },
    structs::{Auth, Tabs, Global, NotificationProps},
};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;
const NAME: &'static str = "Main Page";
#[component(inline_props)]
pub fn MainPage(
    global: Signal<Global>,
    tab: Signal<Tabs>,
    resource: Signal<Option<Persona>>,
    error_message: Signal<NotificationProps>,
) -> View {
    let tab_selector = create_selector(move || tab.get_clone());
    let global_selector = create_selector(move || global.get_clone().auth);
    let r1 = resource.clone();
    create_memo(move || log(NAME, 23, &r1.get_clone()));
    // let resource_selector = create_selector(move || r1.get_clone());

    create_memo(move || match global.get_clone().auth {
        Auth::NotLogged => {
            tab.set_silent(Tabs::Inicio);
            r1.set(None)
        }
        Auth::Logged(login) => block_on(async move {
            r1.set(
                request::<Persona>(
                    format!("api/v1/users/{}", login.id),
                    global,
                    Method::GET,
                    None::<bool>,
                    true,
                )
                .await
                .unwrap(),
            )
        }),
    });

    view! {
        article(id="main_container"){
            (match tab_selector.get_clone(){
                Tabs::Inicio => view!{
                    section(id="main_start"){
                        article(id="main_text"){
                            p(id="community"){"UNA COMUNIDAD"}
                            p(id="community_description"){"BÍBLICA, REFORMADA Y CONFESIONAL HACIENDO ECO DEL EVANGELIO EN LA CIUDAD DE LA PLATA."}
                            button(){"QUÉ CREEMOS"}
                        }
                        article(id="main_pics"){
                            img(src="public/main_frame1.jpg",class="main_img",title="main_img_1")
                            img(src="public/main_frame2.jpg",class="main_img",title="main_img_1")
                            img(src="public/main_frame3.jpg",class="main_img",title="main_img_1")
                            img(src="public/main_frame4.jpg",class="main_img",title="main_img_1")
                            button(){"DOMINGOS 17H"}
                        }
                    }
                    article(id="main_visita"){
                        p(){"ORÁ, VISITÁ, OFRENDÁ O COMPARTÍ."}
                        button(){"DAR"}
                    }
                },
                Tabs::QuienesSomos => view!{QuienesSomos()},
                Tabs::Donar => view!{"Donar"},
                Tabs::Miembros => view!{
                    Miembros(global = global.clone())
                },
                Tabs::Ministerio(ministerio) => view!{Ministerio(global = global, ministerio = ministerio)},
                Tabs::Login =>view!{
                    (match global_selector.get_clone(){
                        Auth::NotLogged => view!{Login(global= global.clone(), error_message = error_message.clone())},
                        Auth::Logged(_) => view!{},
                    })
                }
            })
            // (match resource_selector.get_clone() {
            //     Some(persona) => view!{
            //         UserCard(user = persona)
            //     },
            //     None => view!{}
            // })
            Footer()
        }
    }
}
