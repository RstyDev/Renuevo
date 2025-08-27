use crate::entities::Persona;
use crate::frontend::{
    components::{ActionOnUser, Mode, UserCard},
    lib::request,
    structs::Auth,
};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Miembros(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let m1 = miembros.clone();
    block_on(async move {
        m1.set(
            request::<Vec<Persona>>("api/v1/users/", auth, Method::GET, None::<bool>, true)
                .await
                .unwrap(),
        );
    });

    view! {
        (match miembros.get_clone() {
            Some(miembros) => {
                let iter = miembros.into_iter().map(|m|{
                    let action = create_signal(ActionOnUser::None);
                    view!{li(){UserCard(user=m, mode = Mode::View, action = action)}}}).collect::<Vec<View>>();
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
