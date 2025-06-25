use std::str::FromStr;
use sycamore::prelude::*;
use crate::frontend::structs::Auth;

#[component(inline_props)]
pub fn StateForm(auth: Signal<Auth>
                 // , estado: ReadSignal<EstadoLocal>
) -> View {
    let show_conversion = create_signal(false);
    let show_bautismo = create_signal(false);
    let show_servicio = create_signal(false);
    let conversion = create_signal(String::new());
    view!{
        div(hidden = show_conversion.get()){
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

#[derive(PartialEq, Copy, Clone)]
pub enum EstadoLocal {
    Visitante,
    Nuevo,
    Fundamentos,
    PreMiembro,
    Miembro,
    Diacono,
    Anciano,
}

impl EstadoLocal {
    pub fn from_str(s: &str) -> Self {
        match s{
            "Nuevo" => EstadoLocal::Nuevo,
            "Fundamentos" => EstadoLocal::Fundamentos,
            "PreMiembro" => EstadoLocal::PreMiembro,
            "Miembro" => EstadoLocal::Miembro,
            "Diacono" => EstadoLocal::Diacono,
            "Anciano" => EstadoLocal::Anciano,
            _ => EstadoLocal::Visitante,
        }
    }
}