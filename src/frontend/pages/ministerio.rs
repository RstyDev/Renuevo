use crate::entities::Ministerio;
use crate::frontend::{pages::ministerios::*, structs::Global};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Ministerio(global: Signal<Global>, ministerio: Ministerio) -> View {
    let view = match ministerio {
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
            Presbiterado(global = global)
        },
    };

    view! {
        (view)
    }
}
