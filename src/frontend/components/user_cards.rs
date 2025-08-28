use crate::entities::Persona;
use crate::frontend::{
    components::{ActionOnUser, Mode, UserCard},
    lib::request,
    structs::Auth,
};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;

pub async fn refresh_users(
    miembros: Signal<Vec<Persona>>,
    auth: Signal<crate::frontend::structs::Auth>,
) {
    miembros.set(
        crate::frontend::lib::request::<Vec<Persona>>(
            "api/v1/users/",
            auth,
            Method::GET,
            None::<bool>,
            true,
        )
        .await
        .unwrap()
        .unwrap_or_default(),
    );
}
#[component(inline_props)]
pub fn UserCards(auth: Signal<Auth>, miembros: Signal<Vec<Persona>>, mode: Mode) -> View {
    let m1 = miembros.clone();

    block_on(async move {
        refresh_users(miembros, auth.clone()).await;
    });
    view! {
        ul(id = "miembros"){
            (miembros.get_clone().into_iter().map(|m|{
                let mode = mode.to_owned();
                let action = create_signal(ActionOnUser::None);
                let m2 = m.clone();
                create_memo(move || {
                    let m2 = m2.to_owned();
                    match action.get_clone(){
                        ActionOnUser::Delete => block_on(async move {
                            request::<bool>(
                                format!("api/v1/users/{}",m2.id().unwrap()),
                                auth,
                                Method::DELETE,
                                None::<bool>,
                                false
                            )
                            .await
                            .unwrap();
                            action.set_silent(ActionOnUser::None);
                            refresh_users(m1, auth.clone()).await;
                        }),
                        _=>(),
                    }
                });
                view!{li(){UserCard(user=m, mode = mode.clone(), action=action)}}
            }).collect::<Vec<View>>())
        }
    }
}
