use sycamore::prelude::*;

#[component(inline_props)]
pub fn Palabra() -> View {
    view! {
        p(){"Palabra"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
