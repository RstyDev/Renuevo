use sycamore::prelude::*;

#[component(inline_props)]
pub fn Tesoro() -> View {
    view! {
        p(){"Tesoro"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
