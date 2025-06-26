use std::str::FromStr;
use sycamore::prelude::*;
use crate::entities::{Bautismo, Estado};
use crate::frontend::{structs::Auth, lib::log};

#[component(inline_props)]
pub fn StateForm(auth: Signal<Auth>, estado: Signal<Estado>) -> View {
    log("StateForm",8,&estado.get_clone());
    let conversion = create_signal(String::new());

    view!{
        (match estado.get_clone() {
            Estado::Visitante => view!{},
            Estado::Nuevo => view!{},
            Estado::Fundamentos {conversion, bautismo} | Estado::PreMiembro {conversion, bautismo} => view!{},
            Estado::Miembro {conversion, bautismo, servicio} => view!{},
            Estado::Diacono {conversion, bautismo, servicio} => view!{},
            Estado::Anciano {conversion, bautismo, servicio, tipo} => view!{},
        })
        div(){
            label(r#for="conversion"){"Fecha de Conversion: "}
            input(r#type="date", name="conversion", bind:value=conversion)
        }
    }
}
/*
    Visitante,
    Nuevo,
    Fundamentos {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    PreMiembro {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    Miembro {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Diacono {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Anciano {
        tipo: TipoAnciano,
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
*/
