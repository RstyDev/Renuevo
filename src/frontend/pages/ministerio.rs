use crate::entities::Ministerio;
use crate::frontend::{pages::ministerios::*, structs::Auth};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Ministerio(auth: Signal<Auth>, ministerio: Ministerio) -> View {
    let algo = match ministerio {
        Ministerio::Sonido => view! {
            Sonido()
        },
        Ministerio::Tesoro => view! {
            Tesoro()
        },
        Ministerio::Letras => view! {
            Letras()
        },
        Ministerio::Bienvenida => view! {
            Bienvenida()
        },
        Ministerio::Redes => view! {
            Redes()
        },
        Ministerio::Alabanza => view! {
            Alabanza()
        },
        Ministerio::Misericordia => view! {
            Misericordia()
        },
        Ministerio::Palabra => view! {
            Palabra()
        },
        Ministerio::Presbiterado => view! {
            Presbiterado(auth = auth)
        },
    };

    view! {
        (algo)
    }
}
