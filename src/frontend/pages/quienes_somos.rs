use sycamore::prelude::*;

#[component(inline_props)]
pub fn QuienesSomos() -> View {
    view! {
        h1(){"QUIÉNES SOMOS"}
        h3(){"Visión"}
        p(){"Una comunidad centrada en el evangelio que vive para exaltar a Cristo y hacer discípulos para la gloria de Dios."}
    }
}
