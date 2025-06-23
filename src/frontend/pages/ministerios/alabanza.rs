use sycamore::prelude::*;

#[component(inline_props)]
pub fn Alabanza() -> View {
    view! {
        p(){"Alabanza"}
        button(){"Agregar miembro"}
        button(){"Quitar miembro"}
        button(){"Editar miembro"}
    }
}
