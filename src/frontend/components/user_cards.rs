use crate::entities::Persona;
use crate::frontend::{
    components::user_card::{ActionOnUser, Mode, UserCard},
    lib::{request, refresh_users},
    structs::Global,
};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;


#[component(inline_props)]
pub fn UserCards(global: Signal<Global>, miembros: Signal<Option<Vec<Persona>>>, mode: Mode) -> View {
    let m1 = miembros.clone();

    block_on(async move {
        refresh_users(miembros, global.clone()).await;
    });
    view! {
        (match miembros.get_clone() {
            Some(miembros) => {
                let iter = miembros.into_iter().map(|m|{
                    let mode = mode.to_owned();
                    let action = create_signal(ActionOnUser::None);

                    let m2 = m.clone();
                    create_memo(move || {
                        let m2 = m2.to_owned();

                        match action.get_clone(){
                            ActionOnUser::Delete => block_on(async move {
                                request::<bool>(
                                    format!("api/v1/users/{}",m2.id().unwrap()),
                                    global,
                                    Method::DELETE,
                                    None::<bool>,
                                    false
                                )
                                .await
                                .unwrap();
                                action.set_silent(ActionOnUser::None);
                                refresh_users(m1, global.clone()).await;
                            }),
                            _=>(),
                        }
                    });
                    view!{li(){UserCard(user=m, mode = mode.clone(), action=action)}}
                }).collect::<Vec<View>>();
                view!{
                    ul(id = "miembros"){
                        (iter)
                    }
                }
            },
            None=>view!{},
        })
    }
}
