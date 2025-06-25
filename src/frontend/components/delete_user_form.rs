use crate::entities::Persona;
use crate::frontend::components::{user_card::{ActionOnUser, Mode, UserCard}, user_cards::UserCards};
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
        UserCards(auth = auth, miembros = miembros2, mode = Mode::Delete)
    }
}
