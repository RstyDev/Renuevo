use crate::entities::Persona;
use crate::frontend::components::user_card::{ActionOnUser, Mode, UserCard};
use crate::frontend::{lib::request, structs::Auth};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn EditUserForm(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let m1 = miembros.clone();

    block_on(async move {
        m1.set(
            request::<Vec<Persona>>("api/v1/users/", auth, Method::GET, None::<bool>)
                .await
                .unwrap(),
        );
    });

    view! {
        (match miembros.get_clone() {
            Some(miembros) => {
                let iter = miembros.into_iter().map(|m|{
                    let action = create_signal(ActionOnUser::None);
                    view!{li(){UserCard(user=m, mode = Mode::Edit, action = action)}}}).collect::<Vec<View>>();
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
