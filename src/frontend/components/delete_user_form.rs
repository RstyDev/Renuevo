use crate::entities::Persona;
use crate::frontend::components::{user_card::Mode, user_cards::UserCards};
use crate::frontend::{lib::refresh_users, structs::Global};
use async_std::task::block_on;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn DeleteUserForm(global: Signal<Global>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let miembros2 = miembros.clone();
    block_on(async move {
        refresh_users(miembros, global.clone()).await;
    });

    view! {
        UserCards(global = global, miembros = miembros2, mode = Mode::Delete)
    }
}
