use crate::entities::Persona;
use crate::frontend::components::{user_card::Mode, user_cards::UserCards};
use crate::frontend::{lib::{request, refresh_users}, structs::Auth};
use async_std::task::block_on;
use reqwest::Method;
use sycamore::prelude::*;


#[component(inline_props)]
pub fn DeleteUserForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let miembros2 = miembros.clone();
    block_on(async move {
        refresh_users(miembros, auth.clone()).await;
    });

    view! {
        UserCards(auth = auth, miembros = miembros2, mode = Mode::Delete)
    }
}
