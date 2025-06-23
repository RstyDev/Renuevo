use sycamore::prelude::*;

#[component(inline_props)]
pub fn Misericordia() -> View {
    view! {
        p(){"Misericordia"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
