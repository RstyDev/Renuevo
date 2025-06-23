use sycamore::prelude::*;

#[component(inline_props)]
pub fn Bienvenida() -> View {
    view! {
        p(){"Bienvenida"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
