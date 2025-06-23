use sycamore::prelude::*;

#[component(inline_props)]
pub fn Redes() -> View {
    view! {
        p(){"Redes"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
