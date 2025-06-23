use crate::entities::Persona;
use crate::frontend::components::user_card::{ActionOnUser, Mode, UserCard};
use crate::frontend::{lib::request, structs::Auth};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;

pub async fn refresh_users(miembros: Signal<Option<Vec<Persona>>>, auth: Signal<Auth>) {
    miembros.set(
        request::<Vec<Persona>>("api/v1/users/", auth, Method::GET, None::<bool>)
            .await
            .unwrap(),
    );
}
#[component(inline_props)]
pub fn DeleteUserForm(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let miembros2 = miembros.clone();
    block_on(async move {
        refresh_users(miembros, auth.clone()).await;
    });

    view! {
        (match miembros.get_clone() {
            Some(miembros) => {
                let iter = miembros.into_iter().map(|m|{
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
                                    None::<bool>
                                )
                                .await
                                .unwrap();
                                action.set_silent(ActionOnUser::None);
                                refresh_users(miembros2, auth.clone()).await;
                            }),
                            _=>(),
                        }
                    });
                    view!{li(){UserCard(user=m, mode = Mode::Delete, action=action)}}
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
