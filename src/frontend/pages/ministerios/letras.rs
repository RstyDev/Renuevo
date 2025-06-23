use sycamore::prelude::*;

#[component(inline_props)]
pub fn Letras() -> View {
    view! {
        p(){"Letras"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
