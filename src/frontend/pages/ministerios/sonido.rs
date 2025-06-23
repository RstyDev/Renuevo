use sycamore::prelude::*;

#[component(inline_props)]
pub fn Sonido() -> View {
    view! {
        p(){"Sonido"}
        button(){"Elegir próximo sonidista"}
        button(){"Algo más"}
    }
}
